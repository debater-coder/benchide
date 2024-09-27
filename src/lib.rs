use macroquad::prelude::*;
use crate::Message::{Char, Keypress};
use crate::theme::Theme;

pub mod theme;

pub struct App {
    theme: Theme,
    lines: Vec<String>,
    cursor_position: (usize, usize), // line, character
    pub font: Option<Font>,
    pub font_size: u16
}

impl Default for App {
    fn default() -> Self {
        Self {
            theme: Theme::mocha(),
            lines: vec!["hello".to_owned(), "world".to_owned()],
            cursor_position: (0, 2),
            font: None,
            font_size: 16
        }
    }
}

pub enum Message {
    CursorMove(usize, usize),
    Keypress(KeyCode),
    Char(char),
}

impl App {

    pub fn update(&mut self, message: Message) {
        // match message {
        //
        // }
    }

    pub fn view(&self) -> Vec<Message> {
        let mut messages = vec![];

        clear_background(self.theme.base);

        let mut x = 0.0;
        let mut y = 20.0;


        for (i, line) in self.lines.iter().enumerate() {
            for (j, glyph) in line.chars().enumerate() {
                let dimensions = draw_text_ex(&glyph.to_string(), x, y, TextParams {
                    color: self.theme.text,
                    font: self.font.as_ref(),
                    font_size: self.font_size,
                    ..Default::default()
                });
                x += dimensions.width;

                if (i, j) == self.cursor_position {
                    draw_rectangle(x, y, 2.0, self.font_size as f32, self.theme.text)
                }

            }
            x = 0.0;
            y += self.font_size as f32;
        }

        // Input handling
        for key in get_keys_pressed() {
            messages.push(Keypress(key));
        }

        if let Some(character) = get_char_pressed() {
            messages.push(Char(character))
        }

        messages
    }
}