use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Model {
    #[serde(rename = "claude-3-5-sonnet-20240620")]
    Claude35Sonnet,
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku-20240307")]
    Claude3Haiku,
}

impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Claude35Sonnet => "claude-3-5-sonnet-20240620",
            Self::Claude3Opus => "claude-3-opus-20240229",
            Self::Claude3Sonnet => "claude-3-sonnet-20240229",
            Self::Claude3Haiku => "claude-3-haiku-20240307",
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::Claude35Sonnet
    }
}

impl FromStr for Model {
    type Err = crate::error::AnthropicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "claude-3-5-sonnet-20240620" => Ok(Self::Claude35Sonnet),
            "claude-3-opus-20240229" => Ok(Self::Claude3Opus),
            "claude-3-sonnet-20240229" => Ok(Self::Claude3Sonnet),
            "claude-3-haiku-20240307" => Ok(Self::Claude3Haiku),
            _ => Err(crate::error::AnthropicError::ModelNotSupported(
                s.to_string(),
            )),
        }
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::AnthropicError;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_to_correct_model_names() {
        assert_eq!(Model::Claude35Sonnet.as_str(), "claude-3-5-sonnet-20240620",);
        assert_eq!(Model::Claude3Opus.as_str(), "claude-3-opus-20240229");
        assert_eq!(Model::Claude3Sonnet.as_str(), "claude-3-sonnet-20240229");
        assert_eq!(Model::Claude3Haiku.as_str(), "claude-3-haiku-20240307");
    }

    #[test]
    fn should_deserialize_to_correct_models() {
        assert_eq!(
            Model::Claude35Sonnet,
            Model::from_str("claude-3-5-sonnet-20240620").unwrap(),
        );
        assert_eq!(
            Model::Claude3Opus,
            Model::from_str("claude-3-opus-20240229").unwrap(),
        );
        assert_eq!(
            Model::Claude3Sonnet,
            Model::from_str("claude-3-sonnet-20240229").unwrap(),
        );
        assert_eq!(
            Model::Claude3Haiku,
            Model::from_str("claude-3-haiku-20240307").unwrap(),
        );
    }

    #[test]
    fn should_return_error_for_invalid_model() {
        assert!(matches!(
            Model::from_str("claude-invalid-model"),
            Err(AnthropicError::ModelNotSupported(_))
        ));
    }
}
