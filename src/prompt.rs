use macroquad::prelude::*;
use crate::editor::EditorMessage;
use crate::theme::Theme;
use crate::window::set_fullscreen_camera;

pub(crate) struct Prompt {
    text: String,
    cursor: usize,
}

pub(crate) enum PromptUpdate {
    OpenFile(String),
    CloseActiveFile,
    SaveActiveFile,
    OpenHelp,
    Status(String),
    SaveAs(String),
}

impl Prompt {
    pub(crate) fn new() -> Self {
        Self {
            text: String::from(""),
            cursor: 0,
        }
    }

    pub(crate) fn update(&mut self, message: EditorMessage) -> Option<PromptUpdate> {
        match message {
            EditorMessage::Keypress(key) => {
                match key {
                    KeyCode::Left => {
                        if self.cursor > 0 { self.cursor -= 1; }
                        None
                    }
                    KeyCode::Right => {
                        if self.cursor < self.text.len() { self.cursor += 1 }
                        None
                    }
                    _ => None
                }
            }
            EditorMessage::Char(key) => {
                match key {
                    '\r' => {
                        let update = self.parse_command();
                        self.text.clear();
                        self.cursor = 0;
                        update
                    }
                    '\x08' => {
                        if self.cursor > 0 {
                            self.text.remove(self.cursor - 1);
                            self.cursor -= 1;
                        }
                        None
                    }
                    key => {
                        self.text.insert(self.cursor, key);
                        self.cursor += 1;
                        None
                    }
                }
            }
        }
    }

    pub(crate) fn view(&self, theme: &Theme, font: Option<&Font>) {
        set_fullscreen_camera();
        let mut x = 0.0;
        let y = screen_height() - 8.0;

        let dimensions = draw_text_ex("> ", x, y, TextParams {
            color: theme.text,
            font,
            font_size: 16,
            ..Default::default()
        });

        x += dimensions.width;

        for (i, glyph) in self.text.chars().enumerate() {
            if i == self.cursor {
                draw_rectangle(x, y - 16.0, 2.0, 16.0, theme.rosewater);
            }

            let dimensions = draw_text_ex(&glyph.to_string(), x, y, TextParams {
                color: theme.text,
                font,
                font_size: 16,
                ..Default::default()
            });

            x += dimensions.width;

        }
        if self.cursor == self.text.len() {
            draw_rectangle(x, y - 16.0, 2.0, 16.0, theme.rosewater);
        }

    }

    fn parse_command(&self) -> Option<PromptUpdate> {
        let tokens: Vec<&str> = self.text.split(" ").collect();
        if tokens.len() < 1 {
            return None;
        }

        match tokens[0] {
            "open" => {
                if tokens.len() > 1 {
                    Some(PromptUpdate::OpenFile(tokens[1].to_string()))
                } else {
                    Some(PromptUpdate::Status("Invalid command".to_string()))
                }
            }
            "close" => Some(PromptUpdate::CloseActiveFile),
            "save" => {
                if tokens.len() > 1 {
                    Some(PromptUpdate::SaveAs(tokens[1..].join(" ")))
                } else {
                    Some(PromptUpdate::SaveActiveFile)
                }
            }
            "help" => Some(PromptUpdate::OpenHelp),
            _ => Some(PromptUpdate::Status("Invalid command".to_string()))
        }
    }
}