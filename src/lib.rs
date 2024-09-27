use macroquad::ui::root_ui;
use macroquad::window::clear_background;
use crate::theme::Theme;

pub mod theme;

pub struct App {
    theme: Theme,
    count: u64
}

impl Default for App {
    fn default() -> Self {
        Self {
            theme: Theme::mocha(),
            count: 0
        }
    }
}

pub enum Message {
    Increment
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
        }
    }

    pub fn view(&self) -> Vec<Message> {
        let mut messages = vec![];

        clear_background(self.theme.base);

        root_ui().label(None, "hello megaui");
        if root_ui().button(None, format!("push me {}", self.count)) {
            messages.push(Message::Increment);
        }

        messages
    }
}