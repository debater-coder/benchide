use macroquad::prelude::*;
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
            font_size: 40
        }
    }
}

pub enum Message {
    Keypress(KeyCode),
    Char(char),
}

impl App {

    fn move_cursor(&mut self, key: KeyCode) {
        use KeyCode::*;
        match key {
            Up => {
                if self.cursor_position.0 > 0 {
                    self.cursor_position.0 -= 1
                }
            }
            Left => {
                if self.cursor_position.1 > 0 {
                    self.cursor_position.1 -= 1
                }
            }
            Right => self.cursor_position.1 += 1,
            Down => self.cursor_position.0 += 1,
            _ => {}
        }

        println!("{}", self.lines.len());
        if self.cursor_position.0 >= self.lines.len() {
            self.cursor_position.0 = self.lines.len() - 1
        }

        if self.cursor_position.1 >= self.lines[self.cursor_position.0].len() {
            self.cursor_position.1 = self.lines[self.cursor_position.0].len()
        }

        println!("{:?}", self.cursor_position)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Keypress(key) => self.move_cursor(key),

            Message::Char(character) => {
                let (y, x) = self.cursor_position;

                self.lines[y].insert(x, character);
                self.cursor_position.1 += 1;
            }
        }
    }

    pub fn view(&self) -> Vec<Message> {
        let mut messages = vec![];

        clear_background(self.theme.base);

        let mut x = 0.0;
        let mut y = self.font_size as f32;


        for (i, line) in self.lines.iter().enumerate() {
            for (j, glyph) in line.chars().enumerate() {
                if (i, j) == self.cursor_position {
                    draw_rectangle(x, y - self.font_size as f32, 2.0, self.font_size as f32, self.theme.text)
                }

                let dimensions = draw_text_ex(&glyph.to_string(), x, y, TextParams {
                    color: self.theme.text,
                    font: self.font.as_ref(),
                    font_size: self.font_size,
                    font_scale: 1.0,
                    ..Default::default()
                });

                x += dimensions.width;
            }

            if (i, line.len()) == self.cursor_position {
                draw_rectangle(x, y - self.font_size as f32, 2.0, self.font_size as f32, self.theme.text)
            }

            x = 0.0;
            y += self.font_size as f32;
        }

        // Input handling
        for key in get_keys_pressed() {
            messages.push(Message::Keypress(key));
        }

        if let Some(character) = get_char_pressed() {
            messages.push(Message::Char(character))
        }

        messages
    }
}