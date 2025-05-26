use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    pub content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: String,
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
        assert_eq!(response.content.len(), 1);
        assert_eq!(response.content[0].text, "Hello! How can I help?");
    }
}