use std::env;

use anthropic_ai_sdk::types::message::{
    ContentBlock, MessageClient, MessageError, RequiredMessageParams,
};
use anthropic_ai_sdk::{client::AnthropicClient, types::message::CreateMessageParams};

use crate::message::{Chat, Message, Role};

const SYSTEM_PROMPT: &str = "You are a terminal based AI assistant called Scrawl that helps with general tasks - from writing code, to helping brainstorm to giving feedback. Focus on helping the user *think*. Priotize calling out assumptions or gaps in things that are asked of you.";

#[derive(Debug)]
pub struct ClaudeClientConfig {
    temperature: f32,
}

#[derive(Debug)]
pub struct ClaudeClient {
    client: AnthropicClient,
    config: ClaudeClientConfig,
}

fn get_api_key() -> String {
    env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY is not set")
}

fn get_api_version() -> String {
    env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string())
}

impl ClaudeClient {
    pub fn new(api_key: Option<String>, api_version: Option<String>) -> ClaudeClient {
        let key = api_key.unwrap_or_else(get_api_key);
        let version = api_version.unwrap_or_else(get_api_version);

        let config = ClaudeClientConfig { temperature: 0.7 };

        let client = AnthropicClient::new::<MessageError>(key, version).unwrap();

        ClaudeClient { client, config }
    }

    fn get_role(msg: &Message) -> anthropic_ai_sdk::types::message::Role {
        match msg.role() {
            Role::User => anthropic_ai_sdk::types::message::Role::User,
            Role::Model => anthropic_ai_sdk::types::message::Role::Assistant,
        }
    }

    pub async fn send_message(&self, chat: &Chat) -> String {
        let mut messages: Vec<anthropic_ai_sdk::types::message::Message> = Vec::new();
        for raw_msg in chat.messages() {
            messages.push(anthropic_ai_sdk::types::message::Message::new_text(
                ClaudeClient::get_role(raw_msg),
                raw_msg.content(),
            ));
        }
        let body = CreateMessageParams::new(RequiredMessageParams {
            model: "claude-haiku-4-5-20251001".to_string(),
            messages,
            max_tokens: 1024,
        })
        .with_temperature(self.config.temperature)
        .with_system(SYSTEM_PROMPT);

        let mut resp_text: String = String::new();
        match self.client.create_message(Some(&body)).await {
            Ok(message) => {
                for block in message.content {
                    match block {
                        ContentBlock::Text { text } => {
                            resp_text.push_str(text.as_str());
                        }
                        other => {
                            println!("{:?}", other);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };

        resp_text
    }
}
