use anthropic_rs::{
    api::{
        message::{Content, ContentType, Message, MessageRequest, Role},
        stream::StreamEvent,
    },
    client::Client,
    config::Config,
    models::model::Model,
};
use futures_util::StreamExt;
use std::io::Write;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("environment variable ANTHROPIC_API_KEY should be defined");

    let config = Config::new(api_key);
    let client = Client::new(config).unwrap();

    let message = MessageRequest {
        model: Model::Claude35Sonnet,
        stream: Some(true),
        max_tokens: 1024,
        messages: vec![Message {
            role: Role::User,
            content: vec![Content {
                content_type: ContentType::Text,
                text: "Explain the theory of relativity".to_string(),
            }],
        }],
        ..Default::default()
    };

    let mut stream = client.stream_message(message.clone()).await.unwrap();

    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => match event {
                StreamEvent::ContentBlockDelta(content) => {
                    print!("{}", content.delta.text);
                    std::io::stdout().flush().unwrap();
                }
                StreamEvent::MessageStop => break,
                _ => {}
            },
            Err(err) => println!("{}", err),
        }
    }
}
