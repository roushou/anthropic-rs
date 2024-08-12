use serde::{Deserialize, Serialize};

use crate::models::model::Model;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: Vec<Content>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Content {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    pub model: Model,
    pub max_tokens: u32,
    pub messages: Vec<Message>,
    pub metadata: Option<MessageMetadata>,
    pub stop_sequences: Option<Vec<String>>,
}

impl MessageRequest {
    pub fn new(model: Model, max_tokens: u32, messages: Vec<Message>) -> Self {
        Self {
            model,
            max_tokens,
            messages,
            ..Default::default()
        }
    }

    pub fn with_metadata(mut self, metadata: MessageMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }
}

impl Default for MessageRequest {
    fn default() -> Self {
        Self {
            model: Model::Claude35Sonnet,
            max_tokens: 0,
            messages: Vec::new(),
            metadata: None,
            stop_sequences: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageMetadata {
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub role: RoleResponse,
    pub content: Vec<Content>,
    pub model: Model,
    pub stop_reason: StopReason,
    pub stop_sequence: Option<String>,
    pub usage: TokenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoleResponse {
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_set_metadata() {
        let request = MessageRequest::default();
        assert_eq!(request.metadata, None);

        let metadata = MessageMetadata {
            user_id: Some("user-id".to_string()),
        };
        let request = request.with_metadata(metadata.clone());
        assert_eq!(request.metadata, Some(metadata));
    }

    #[test]
    fn should_set_stop_sequences() {
        let request = MessageRequest::default();
        assert_eq!(request.stop_sequences, None);

        let stop_sequences: Vec<String> = vec!["foo".to_string(), "bar".to_string()];
        let request = request.with_stop_sequences(stop_sequences.clone());
        assert_eq!(request.stop_sequences, Some(stop_sequences));
    }

    #[test]
    fn should_serialize_message() {
        let message = Message {
            role: Role::User,
            content: vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }],
        };
        assert_eq!(
            serde_json::to_value(&message).unwrap(),
            serde_json::json!({
                "role": "user",
                "content": [{
                    "type": "text",
                    "text": "Hello World"
                }],
            })
        );

        let message = Message {
            role: Role::Assistant,
            content: vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }],
        };
        assert_eq!(
            serde_json::to_value(&message).unwrap(),
            serde_json::json!({
                "role": "assistant",
                "content": [{
                    "type": "text",
                    "text": "Hello World"
                }],
            })
        );
    }

    #[test]
    fn should_deserialize_message() {
        let json = serde_json::json!({
            "role": "user",
            "content": [{
                "type": "text",
                "text": "Hello World",
            }]
        });
        let message: Message = serde_json::from_value(json).unwrap();
        assert_eq!(message.role, Role::User);
        assert_eq!(
            message.content,
            vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }]
        );

        let json = serde_json::json!({
            "role": "assistant",
            "content": [{
                "type": "text",
                "text": "Hello World",
            }]
        });
        let message: Message = serde_json::from_value(json).unwrap();
        assert_eq!(message.role, Role::Assistant);
        assert_eq!(
            message.content,
            vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }]
        );
    }
}
