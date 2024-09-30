use macroquad::prelude::*;
use crate::editor::EditorMessage;

pub(crate) struct Command {
    text: String,
    cursor: usize,
}

pub(crate) enum CommandUpdate {
    OpenFile(String),
    CloseActiveFile,
    SaveActiveFile,
    OpenHelp,
    Status(String),
    SaveAs(String),
}

impl Command {
    pub(crate) fn new() -> Self {
        Self {
            text: String::from(""),
            cursor: 0,
        }
    }

    pub(crate) fn update(&mut self, message: EditorMessage) -> Option<CommandUpdate> {
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

                        update
                    }
                    '\x08' => {
                        self.text.remove(self.cursor - 1);
                        None
                    }
                    key => {
                        self.text.insert(self.cursor, key);
                        None
                    }
                }
            }
        }
    }

    pub(crate) fn view() {}

    fn parse_command(&self) -> Option<CommandUpdate> {
        let tokens: Vec<&str> = self.text.split(" ").collect();
        if tokens.len() < 1 {
            return None;
        }

        match tokens[0] {
            "open" => {
                if tokens.len() > 1 {
                    Some(CommandUpdate::OpenFile(tokens[1].to_string()))
                } else {
                    Some(CommandUpdate::Status("Invalid command".to_string()))
                }
            }
            "close" => Some(CommandUpdate::CloseActiveFile),
            "save" => {
                if tokens.len() > 1 {
                    Some(CommandUpdate::SaveAs(tokens[1..].join(" ")))
                } else {
                    Some(CommandUpdate::SaveActiveFile)
                }
            }
            "help" => Some(CommandUpdate::OpenHelp),
            _ => Some(CommandUpdate::Status("Invalid command".to_string()))
        }
    }
}