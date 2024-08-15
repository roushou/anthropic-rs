# Anthropic Rust SDK

This is an unofficial Rust SDK for the Anthropic API.

## Basic usage

```rs
use anthropic_rs::{
    api::message::{Content, ContentType, Message, MessageRequest, Role},
    client::Client,
    config::Config,
    models::model::Model,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("environment variable ANTHROPIC_API_KEY should be defined");

    let config = Config::new(api_key);
    let client = Client::new(config).unwrap();

    let message = MessageRequest {
        model: Model::Claude35Sonnet,
        max_tokens: 1024,
        messages: vec![Message {
            role: Role::User,
            content: vec![Content {
                content_type: ContentType::Text,
                text: "Hello World".to_string(),
            }],
        }],
        ..Default::default()
    };

    let result = client.create_message(message.clone()).await.unwrap();
    println!("{:?}", result);
}
```
