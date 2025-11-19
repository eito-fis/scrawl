use ratatui::crossterm::event::{self, Event, KeyCode};

mod clients;
mod message;

use clients::claude::ClaudeClient;
use message::{Chat, Message, Role};

#[tokio::main]
async fn main() {
    let mut chat = Chat::default();
    chat.push(Message::default());

    let client = ClaudeClient::new(Option::None, Option::None);

    let mut terminal = ratatui::init();

    loop {
        terminal
            .draw(|frame| {
                frame.render_widget(&chat, frame.area());
            })
            .expect("Failed to render");

        let event = event::read().expect("Failed to read");
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    let model_message = client.send_message(&chat).await;
                    chat.push(Message::new(model_message, Role::Model));
                    chat.push(Message::new(String::default(), Role::User));
                }
                other => {
                    chat.handle_input(other);
                }
            };
        };
    }
    ratatui::restore();
}
