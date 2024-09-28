use inkjet::Highlighter;
use macroquad::prelude::*;
use crate::editor::{Editor, EditorMessage};
use crate::theme::Theme;

pub mod theme;
mod editor;
mod window;

pub struct App {
    theme: Theme,
    pub font: Option<Font>,
    pub font_size: u16,
    editor: Editor,
    highlighter: Highlighter
}

impl Default for App {
    fn default() -> Self {
        Self {
            theme: Theme::mocha(),
            font: None,
            font_size: 16,
            editor: Editor::new(Rect::new(20.0, 20.0, 400.0, 400.0)),
            highlighter: Highlighter::new()
        }
    }
}

pub enum Message {
    Edit(EditorMessage),
}

impl App {

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edit(edit) => self.editor.update(edit, &mut self.highlighter, &self.theme)
        }
    }

    fn handle_input(&self, messages: &mut Vec<Message>) {
        for key in get_keys_pressed() {
            messages.push(Message::Edit(EditorMessage::Keypress(key)));
        }

        if let Some(character) = get_char_pressed() {
            messages.push(Message::Edit(EditorMessage::Char(character)))
        }
    }

    pub fn view(&self) -> Vec<Message> {
        let mut messages = vec![];

        clear_background(self.theme.base);
        set_default_camera();
        // draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width() - 160., 32., 32., WHITE);

        self.editor.view(&self.theme, self.font.as_ref(), self.font_size);

        self.handle_input(&mut messages);

        messages
    }
}