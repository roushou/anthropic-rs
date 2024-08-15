use core::fmt;
use futures_util::StreamExt;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
    Method, RequestBuilder, Url,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    api::{
        message::{MessageRequest, MessageResponse},
        stream::StreamEvent,
    },
    config::Config,
    error::{AnthropicError, ApiErrorResponse},
};

pub struct Client {
    api_key: String,
    api_version: ApiVersion,
    anthropic_version: AnthropicVersion,
    base_url: Url,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(config: Config) -> Result<Self, AnthropicError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(config.api_key.as_str())
                .map_err(AnthropicError::InvalidHeaderValue)?,
        );
        headers.insert(
            "anthropic-version",
            HeaderValue::from_str(&config.anthropic_version.to_string())
                .map_err(AnthropicError::InvalidHeaderValue)?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let base_url = Url::parse(&config.base_url)
            .map_err(|err| AnthropicError::UrlParse(err.to_string()))?
            .join(format!("{}/", config.api_version).as_str())
            .map_err(|err| AnthropicError::UrlParse(err.to_string()))?;

        Ok(Self {
            anthropic_version: config.anthropic_version,
            api_key: config.api_key,
            api_version: config.api_version,
            base_url,
            http_client,
        })
    }

    pub fn anthropic_version(&self) -> &AnthropicVersion {
        &self.anthropic_version
    }

    pub fn api_key(&self) -> &str {
        self.api_key.as_str()
    }

    pub fn api_version(&self) -> &ApiVersion {
        &self.api_version
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, AnthropicError> {
        let url = self
            .base_url
            .join(path)
            .map_err(|err| AnthropicError::UrlParse(err.to_string()))?;
        Ok(self.http_client.request(method, url))
    }

    pub async fn create_message(
        &self,
        payload: MessageRequest,
    ) -> Result<MessageResponse, AnthropicError> {
        let response = self
            .request(Method::POST, "messages")?
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            match serde_json::from_str::<ApiErrorResponse>(&error) {
                Ok(api_error) => return Err(AnthropicError::Api(api_error)),
                Err(err) => return Err(AnthropicError::JsonDeserialize(err)),
            }
        }

        response
            .json::<MessageResponse>()
            .await
            .map_err(AnthropicError::from)
    }

    pub async fn stream_message(&self, request: MessageRequest) -> Result<(), AnthropicError> {
        let response = self
            .request(Method::POST, "messages")?
            .header(ACCEPT, "text/event-stream")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            match serde_json::from_str::<ApiErrorResponse>(&error) {
                Ok(api_error) => return Err(AnthropicError::Api(api_error)),
                Err(err) => return Err(AnthropicError::JsonDeserialize(err)),
            }
        }

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let chunk_str = std::str::from_utf8(&chunk).unwrap();

            for event in chunk_str.split("\n\n") {
                let event = event.trim();
                if event.is_empty() {
                    continue;
                }

                let data: Vec<&str> = event.split("\n").collect();

                if let Some(content) = data[1].strip_prefix("data: ") {
                    let content = StreamEvent::from_str(content)?;
                    if let StreamEvent::ContentBlockDelta(content) = content {
                        print!("{}", content.delta.text);
                    }
                };
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnthropicVersion {
    Latest,
    Initial,
}

impl Default for AnthropicVersion {
    fn default() -> Self {
        Self::Latest
    }
}

impl fmt::Display for AnthropicVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Latest => write!(f, "2023-06-01"),
            Self::Initial => write!(f, "2023-01-01"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApiVersion {
    V1,
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::V1
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V1 => write!(f, "v1"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Invalid API version: {0}")]
pub struct ApiVersionError(String);

impl FromStr for ApiVersion {
    type Err = ApiVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1" => Ok(Self::V1),
            _ => Err(ApiVersionError(s.to_string())),
        }
    }
}
