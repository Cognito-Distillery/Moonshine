use serde::{Deserialize, Serialize};

use crate::ai::embedding::{EmbeddingConfig, EmbeddingProvider};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationCandidate {
    pub source_id: String,
    pub source_summary: String,
    pub target_id: String,
    pub target_summary: String,
}

#[derive(Debug, Clone)]
pub struct ExtractedRelation {
    pub source_id: String,
    pub target_id: String,
    pub relation: String,
    pub confidence: f64,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    temperature: f64,
    response_format: ResponseFormat,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct AIResponse {
    relations: Vec<AIRelation>,
}

#[derive(Deserialize)]
struct AIRelation {
    #[serde(rename = "sourceId")]
    source_id: String,
    #[serde(rename = "targetId")]
    target_id: String,
    relation: Option<String>,
    confidence: f64,
}

const VALID_RELATIONS: &[&str] = &["RELATED_TO", "SUPPORTS", "CONFLICTS_WITH"];

pub async fn extract_relations(
    config: &EmbeddingConfig,
    candidates: Vec<RelationCandidate>,
) -> Result<Vec<ExtractedRelation>, String> {
    if candidates.is_empty() {
        return Ok(vec![]);
    }

    let mut results = Vec::new();

    // Process in batches of 5
    for batch in candidates.chunks(5) {
        match extract_batch(config, batch).await {
            Ok(extracted) => results.extend(extracted),
            Err(e) => {
                log::warn!("Relationship extraction batch failed, skipping: {}", e);
            }
        }
    }

    Ok(results)
}

async fn extract_batch(
    config: &EmbeddingConfig,
    batch: &[RelationCandidate],
) -> Result<Vec<ExtractedRelation>, String> {
    let pairs_json = serde_json::to_string(batch).map_err(|e| e.to_string())?;

    let url = match config.provider {
        EmbeddingProvider::OpenAI => "https://api.openai.com/v1/chat/completions",
        EmbeddingProvider::Gemini => {
            "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions"
        }
    };

    let request = ChatRequest {
        model: config.chat_model.clone(),
        temperature: 0.1,
        response_format: ResponseFormat {
            format_type: "json_object".to_string(),
        },
        messages: vec![
            Message {
                role: "system".to_string(),
                content: r#"You classify relationships between knowledge items in a personal knowledge base.

For each pair, determine the relationship type:
- "RELATED_TO": items share a common topic, context, or are part of the same domain/project
- "SUPPORTS": source reinforces, extends, or provides evidence for target
- "CONFLICTS_WITH": source contradicts or creates tension with target
- null: completely unrelated items with no connection

Return JSON: { "relations": [{ "sourceId": string, "targetId": string, "relation": string | null, "confidence": number (0.0-1.0) }] }

These items were pre-filtered by semantic similarity, so most pairs likely have some connection. Use RELATED_TO generously for items in the same domain. Only use null when items are truly unrelated."#.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: pairs_json,
            },
        ],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Chat API request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Chat API error {}: {}", status, body));
    }

    let result: ChatResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse chat response: {}", e))?;

    let content = result
        .choices
        .first()
        .and_then(|c| c.message.content.as_ref())
        .ok_or_else(|| "No response content".to_string())?;

    log::info!("AI response: {}", content);

    let parsed: AIResponse =
        serde_json::from_str(content).map_err(|e| format!("Failed to parse AI response: {}", e))?;

    // Build set of valid IDs from the input candidates
    let valid_ids: std::collections::HashSet<&str> = batch
        .iter()
        .flat_map(|c| [c.source_id.as_str(), c.target_id.as_str()])
        .collect();

    Ok(parsed
        .relations
        .into_iter()
        .filter(|r| {
            r.relation.is_some()
                && VALID_RELATIONS.contains(&r.relation.as_deref().unwrap_or(""))
                && r.confidence > 0.0
                && valid_ids.contains(r.source_id.as_str())
                && valid_ids.contains(r.target_id.as_str())
        })
        .map(|r| ExtractedRelation {
            source_id: r.source_id,
            target_id: r.target_id,
            relation: r.relation.unwrap(),
            confidence: r.confidence,
        })
        .collect())
}
