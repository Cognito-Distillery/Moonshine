use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::db;

// --- Provider types ---

#[derive(Debug, Clone, PartialEq)]
pub enum EmbeddingProvider {
    OpenAI,
    Gemini,
}

impl EmbeddingProvider {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "openai" => Ok(Self::OpenAI),
            "gemini" => Ok(Self::Gemini),
            _ => Err(format!("Unknown embedding provider: {}", s)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAI => "openai",
            Self::Gemini => "gemini",
        }
    }
}

#[derive(Debug, Clone)]
pub enum EmbeddingTaskType {
    Document,
    Query,
}

#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    pub provider: EmbeddingProvider,
    pub api_key: String,
    pub embedding_model: String,
    pub chat_model: String,
}

pub fn default_embedding_model(provider: &EmbeddingProvider) -> &'static str {
    match provider {
        EmbeddingProvider::OpenAI => "text-embedding-3-small",
        EmbeddingProvider::Gemini => "gemini-embedding-001",
    }
}

pub fn default_chat_model(provider: &EmbeddingProvider) -> &'static str {
    match provider {
        EmbeddingProvider::OpenAI => "gpt-4o-mini",
        EmbeddingProvider::Gemini => "gemini-2.0-flash",
    }
}

pub fn resolve_embedding_config(conn: &Connection) -> Result<EmbeddingConfig, String> {
    let provider_str = db::settings::get_setting(conn, "embedding_provider")?
        .unwrap_or_else(|| "openai".to_string());
    let provider = EmbeddingProvider::from_str(&provider_str)?;

    let key_name = match provider {
        EmbeddingProvider::OpenAI => "openai_api_key",
        EmbeddingProvider::Gemini => "gemini_api_key",
    };

    let api_key = db::settings::get_setting(conn, key_name)?
        .ok_or_else(|| format!("{} API key not configured", provider.as_str()))?;

    let embedding_model = db::settings::get_setting(conn, "embedding_model")?
        .unwrap_or_else(|| default_embedding_model(&provider).to_string());

    let chat_model = db::settings::get_setting(conn, "chat_model")?
        .unwrap_or_else(|| default_chat_model(&provider).to_string());

    Ok(EmbeddingConfig {
        provider,
        api_key,
        embedding_model,
        chat_model,
    })
}

// --- Main entry point ---

pub async fn generate_embeddings(
    config: &EmbeddingConfig,
    texts: Vec<String>,
    task_type: EmbeddingTaskType,
) -> Result<Vec<Option<Vec<f32>>>, String> {
    if texts.is_empty() {
        return Ok(vec![]);
    }

    match config.provider {
        EmbeddingProvider::OpenAI => {
            generate_openai_embeddings(&config.api_key, &config.embedding_model, texts).await
        }
        EmbeddingProvider::Gemini => {
            generate_gemini_embeddings(&config.api_key, &config.embedding_model, texts, task_type)
                .await
        }
    }
}

// --- OpenAI ---

#[derive(Serialize)]
struct OpenAIEmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbeddingData>,
}

#[derive(Deserialize)]
struct OpenAIEmbeddingData {
    embedding: Vec<f32>,
}

async fn generate_openai_embeddings(
    api_key: &str,
    model: &str,
    texts: Vec<String>,
) -> Result<Vec<Option<Vec<f32>>>, String> {
    let client = reqwest::Client::new();
    let mut all_results: Vec<Option<Vec<f32>>> = Vec::with_capacity(texts.len());

    // OpenAI batch limit: 2048 per request
    for chunk in texts.chunks(2048) {
        let request = OpenAIEmbeddingRequest {
            model: model.to_string(),
            input: chunk.to_vec(),
        };

        let response = client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Embedding API request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Embedding API error {}: {}", status, body));
        }

        let result: OpenAIEmbeddingResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse embedding response: {}", e))?;

        for d in result.data {
            all_results.push(Some(d.embedding));
        }
    }

    Ok(all_results)
}

// --- Gemini ---

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiBatchRequest {
    requests: Vec<GeminiEmbedRequest>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiEmbedRequest {
    model: String,
    content: GeminiContent,
    task_type: String,
    output_dimensionality: u32,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Deserialize)]
struct GeminiBatchResponse {
    embeddings: Vec<GeminiEmbeddingValues>,
}

#[derive(Deserialize)]
struct GeminiEmbeddingValues {
    values: Vec<f32>,
}

async fn generate_gemini_embeddings(
    api_key: &str,
    model: &str,
    texts: Vec<String>,
    task_type: EmbeddingTaskType,
) -> Result<Vec<Option<Vec<f32>>>, String> {
    let client = reqwest::Client::new();
    let task_type_str = match task_type {
        EmbeddingTaskType::Document => "RETRIEVAL_DOCUMENT",
        EmbeddingTaskType::Query => "RETRIEVAL_QUERY",
    };

    let model_path = if model.starts_with("models/") {
        model.to_string()
    } else {
        format!("models/{}", model)
    };

    let mut all_results: Vec<Option<Vec<f32>>> = Vec::with_capacity(texts.len());

    // Gemini batch limit: 100 per request
    for chunk in texts.chunks(100) {
        let requests: Vec<GeminiEmbedRequest> = chunk
            .iter()
            .map(|text| GeminiEmbedRequest {
                model: model_path.clone(),
                content: GeminiContent {
                    parts: vec![GeminiPart { text: text.clone() }],
                },
                task_type: task_type_str.to_string(),
                output_dimensionality: 1536,
            })
            .collect();

        let batch_request = GeminiBatchRequest { requests };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/{}:batchEmbedContents",
            model_path
        );
        let response = client
            .post(&url)
            .header("x-goog-api-key", api_key)
            .json(&batch_request)
            .send()
            .await
            .map_err(|e| format!("Gemini Embedding API request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Gemini Embedding API error {}: {}", status, body));
        }

        let result: GeminiBatchResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Gemini embedding response: {}", e))?;

        for emb in result.embeddings {
            all_results.push(Some(emb.values));
        }
    }

    Ok(all_results)
}
