use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget, WidgetRef, Wrap},
};

const MESSAGE_Y_PADDING: u16 = 1;
const USER_LEFT_PADDING: u16 = 6;

#[derive(Default, Debug)]
pub enum Role {
    #[default]
    User,
    Model,
}

#[derive(Default, Debug)]
pub struct Message {
    content: String,
    role: Role,
}

impl Message {
    pub fn new(content: String, role: Role) -> Message {
        Message { content, role }
    }

    fn insert(&mut self, c: char) {
        self.content.push(c);
    }

    fn backspace(&mut self) {
        self.content.pop();
    }

    fn as_paragraph<'a>(&'a self, mut scroll_top: u16) -> Paragraph<'a> {
        scroll_top = scroll_top.saturating_sub(1);
        let mut block = Block::default();
        let title_text = match self.role {
            Role::User => "User",
            Role::Model => "Scrawl",
        };
        let title = Line::from(title_text.bold());
        if scroll_top == 0 {
            block = block.borders(Borders::ALL).title(title.left_aligned());
        } else {
            block = block.borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);
        }

        Paragraph::new(self.content.as_str())
            .left_aligned()
            .wrap(Wrap { trim: false })
            .block(block)
            .scroll((scroll_top, 0))
    }

    fn height(&self, width: u16) -> u16 {
        let paragraph = self.as_paragraph(0);
        let height = paragraph.line_count(width - 2);
        u16::try_from(height).expect("Failed to cast height")
    }

    fn left_pad(&self) -> u16 {
        match self.role {
            Role::User => USER_LEFT_PADDING,
            Role::Model => 0,
        }
    }

    fn width(&self, width: u16) -> u16 {
        width - self.left_pad()
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}

#[derive(Default, Debug)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        let Some(last_message) = self.messages.last_mut() else {
            return;
        };
        match key {
            KeyCode::Char(c) => last_message.insert(c),
            KeyCode::Backspace => last_message.backspace(),
            _ => {}
        }
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }
}

impl WidgetRef for Chat {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let total_width = area.width;
        let total_height = area.height;
        let mut remaining_height = total_height;

        for msg in self.messages.iter().rev() {
            let width = msg.width(total_width);
            let height = msg.height(width);
            let y: u16 = remaining_height.saturating_sub(height);
            // 0 if no cutoff lines, some value otherwise
            // Handled cleanly by message.as_paragraph
            let cutoff_lines = height.saturating_sub(remaining_height);

            msg.as_paragraph(cutoff_lines).render(
                Rect {
                    x: msg.left_pad(),
                    y,
                    width,
                    height: height - cutoff_lines,
                },
                buf,
            );

            remaining_height = y.saturating_sub(MESSAGE_Y_PADDING);
            if remaining_height == 0 {
                break;
            }
        }
    }
}
