use macroquad::prelude::*;
use crate::theme::Theme;
use crate::window::set_camera_window;

pub(crate) struct Editor {
    lines: Vec<String>,
    cursor_position: (usize, usize), // line, character
    window: Rect
}

pub(crate) enum EditorMessage {
    Keypress(KeyCode),
    Char(char),
}

impl Editor {
    pub fn new(window: Rect) -> Self {
        Self {
            lines: vec!["".to_owned()],
            cursor_position: (0, 0),
            window
        }
    }

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

        if self.cursor_position.0 >= self.lines.len() {
            self.cursor_position.0 = self.lines.len() - 1
        }

        if self.cursor_position.1 >= self.lines[self.cursor_position.0].len() {
            self.cursor_position.1 = self.lines[self.cursor_position.0].len()
        }

    }

    fn insert_str_single_line(&mut self, string: &str) {
        self.lines[self.cursor_position.0].insert_str(self.cursor_position.1, string);
        self.cursor_position.1 += string.len();
    }

    pub fn update(&mut self, message: EditorMessage) {
        match message {
            EditorMessage::Keypress(key) =>
                match key {
                    KeyCode::Up | KeyCode::Left | KeyCode::Right | KeyCode::Down => self.move_cursor(key),
                    KeyCode::Tab => {
                        self.insert_str_single_line("    ")
                    }
                    _ => {}
                },

            EditorMessage::Char(character) => {
                let (y, x) = self.cursor_position;

                match character {
                    '\x08' => {
                        if x > 0 {
                            self.lines[y].remove(x - 1);
                            self.cursor_position.1 -= 1;
                        } else if y > 0 {
                            let original_line = &self.lines[y].clone();
                            let restored_position = self.lines[y - 1].len();
                            self.lines[y - 1] += original_line;
                            self.lines.remove(y);
                            self.cursor_position = (y - 1, restored_position);
                        }
                    }

                    '\t' => {
                        // Handled by Message::Keypress
                    }

                    '\r' => {
                        let original_line = self.lines[y].clone();
                        let (first, last) = original_line.split_at(x);

                        self.lines[y] = first.to_owned();

                        self.lines.insert(y + 1, last.to_owned());
                        self.cursor_position = (y + 1, 0);
                    }
                    _ => {
                        self.lines[y].insert(x, character);
                        self.cursor_position.1 += 1;
                    }
                }

            }
        }
    }

    pub fn view(&self, theme: &Theme, font: Option<&Font>, font_size: u16) {
        draw_rectangle(self.window.x, self.window.y, self.window.w, self.window.h, theme.surface0);
        set_camera_window(self.window, vec2(0.0, 0.0));

        let mut x = 0.0;
        let mut y = font_size as f32;


        for (i, line) in self.lines.iter().enumerate() {
            for (j, glyph) in line.chars().enumerate() {
                if (i, j) == self.cursor_position {
                    draw_rectangle(x, y - font_size as f32, 2.0, font_size as f32, theme.text)
                }

                let dimensions = draw_text_ex(&glyph.to_string(), x, y, TextParams {
                    color: theme.text,
                    font,
                    font_size,
                    ..Default::default()
                });

                x += dimensions.width;
            }

            if (i, line.len()) == self.cursor_position {
                draw_rectangle(x, y - font_size as f32, 2.0, font_size as f32, theme.text)
            }

            x = 0.0;
            y += font_size as f32;
        }

        set_default_camera();
    }
}