use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::message::{
    ContentBlock, CreateMessageParams, Message, MessageClient, MessageError, RequiredMessageParams,
    Role,
};
use std::env;
use std::io::{self, Write};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new::<MessageError>(api_key, api_version).unwrap();

    let mut user_input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut messages: Vec<Message> = Vec::new();

    println!("Starting chat...");
    println!("----------------------------------------");

    loop {
        // Get input
        print!("Input: ");
        stdout.flush().expect("Failed to flush stdout");
        stdin
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let trimmed_input = user_input.trim();
        println!();

        messages.push(Message::new_text(Role::User, trimmed_input));

        let body = CreateMessageParams::new(RequiredMessageParams {
            model: "claude-haiku-4-5-20251001".to_string(),
            messages: messages.clone(),
            max_tokens: 1024,
        })
        .with_temperature(0.7)
        .with_system("You are a terminal based AI assistant that helps with general tasks - from writing code, to helping brainstorm to giving feedback. Focus on helping the user *think*. Priotize calling out assumptions or gaps in things that are asked of you.");

        match client.create_message(Some(&body)).await {
            Ok(message) => {
                for block in message.content {
                    match block {
                        ContentBlock::Text { text } => {
                            println!("{}\n", text);
                            messages.push(Message::new_text(Role::Assistant, text));
                        }
                        other => {
                            println!("{:?}", other);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
