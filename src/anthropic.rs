use anyhow::Result;
use reqwest::{Client, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_VERSION: &str = "2023-06-01";
const ANTHROPIC_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "claude-opus-4-20250514")]
    ClaudeOpus4,
    #[serde(rename = "claude-sonnet-4-20250514")]
    ClaudeSonnet4,
    #[serde(rename = "claude-3-7-sonnet-20250219")]
    ClaudeSonnet37,
    #[serde(rename = "claude-3-5-haiku-20241022")]
    ClaudeHaiku35,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct MessageRequest {
    pub model: Model,
    pub max_tokens: u32,
    pub messages: Vec<ChatMessage>,
}

impl MessageRequest {
    pub fn from_messages(messages: Vec<ChatMessage>) -> Self {
        Self {
            model: Model::ClaudeSonnet4,
            max_tokens: 1024,
            messages,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    #[serde(rename = "content")]
    pub content_blocks: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
pub enum ContentBlockType {
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub block_type: ContentBlockType,
    pub text: String,
}

pub struct AnthropicClient {
    api_key: String,
    client: Client,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        AnthropicClient {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn send_message(&self, request: MessageRequest) -> Result<MessageResponse> {
        let response = self
            .client
            .post(ANTHROPIC_ENDPOINT)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_API_VERSION)
            .header(CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await?;

        let message_response: MessageResponse = response.json().await?;

        Ok(message_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_request_serialization() {
        let message = ChatMessage {
            role: Role::User,
            content: "Hello!".to_string(),
        };

        let request = MessageRequest {
            model: Model::ClaudeSonnet4,
            max_tokens: 1024,
            messages: vec![message],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("claude-sonnet-4-20250514"));
        assert!(!json.contains("claude-sonnet-3-20250514"));
        assert!(json.contains("user"));
        assert!(json.contains("Hello!"));
    }

    #[test]
    fn test_message_response_deserialization() {
        let response_json = r#"{
              "content": [{"type": "text", "text": "Hello! How can I help?"}]
          }"#;

        let response: MessageResponse = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.content_blocks.len(), 1);
        assert_eq!(response.content_blocks[0].text, "Hello! How can I help?");
    }
}
