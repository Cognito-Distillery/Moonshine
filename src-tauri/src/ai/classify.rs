use serde::Deserialize;
use serde::Serialize;

use crate::ai::embedding::{EmbeddingConfig, EmbeddingProvider};

#[derive(Debug, Clone, Deserialize)]
pub struct ClassifyResponse {
    #[serde(rename = "type")]
    pub mash_type: String,
    pub summary: String,
    pub context: Option<String>,
    pub memo: Option<String>,
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
struct ChatApiResponse {
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

const SYSTEM_PROMPT: &str = r#"You are a knowledge classification assistant for a personal knowledge base.
Given raw text input from the user, extract and return a structured JSON object.

Rules:
- "type": one of "결정" (decision made), "문제" (problem/issue to solve), "인사이트" (insight/discovery), "질문" (question needing discussion)
- "summary": a concise one-line summary of the input (same language as input)
- "context": background context extracted from the input, or null if none
- "memo": any additional notes or details, or null if none

Return JSON: { "type": string, "summary": string, "context": string | null, "memo": string | null }

If there is no distinct context or memo extractable from the input, set them to null.
Do not fabricate information — only extract what is actually present in the text."#;

const VALID_TYPES: &[&str] = &["결정", "문제", "인사이트", "질문"];

pub async fn classify_text(
    config: &EmbeddingConfig,
    text: &str,
) -> Result<ClassifyResponse, String> {
    let url = match config.provider {
        EmbeddingProvider::OpenAI => "https://api.openai.com/v1/chat/completions",
        EmbeddingProvider::Gemini => {
            "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions"
        }
    };

    let request = ChatRequest {
        model: config.chat_model.clone(),
        temperature: 0.3,
        response_format: ResponseFormat {
            format_type: "json_object".to_string(),
        },
        messages: vec![
            Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: text.to_string(),
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

    let result: ChatApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse chat response: {}", e))?;

    let content = result
        .choices
        .first()
        .and_then(|c| c.message.content.as_ref())
        .ok_or_else(|| "No response content".to_string())?;

    log::info!("AI classify response: {}", content);

    let parsed: ClassifyResponse = serde_json::from_str(content)
        .map_err(|e| format!("Failed to parse AI classification response: {}", e))?;

    if !VALID_TYPES.contains(&parsed.mash_type.as_str()) {
        return Err(format!(
            "Invalid mash type from AI: '{}'. Expected one of: 결정, 문제, 인사이트, 질문",
            parsed.mash_type
        ));
    }

    Ok(parsed)
}
