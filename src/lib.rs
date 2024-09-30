use std::collections::HashMap;
use inkjet::Highlighter;
use macroquad::prelude::*;
use uuid::Uuid;
use crate::editor::{Editor, EditorMessage};
use crate::prompt::{Prompt, PromptUpdate};
use crate::theme::Theme;

pub mod theme;
mod editor;
mod window;
mod prompt;

pub struct App {
    theme: Theme,
    pub font: Option<Font>,
    editors: HashMap<Uuid, Editor>,
    highlighter: Highlighter,
    focused: Option<Uuid>,
    move_target: Option<Uuid>,
    prompt_focused: bool,
    released: bool,
    prompt: Prompt
}

impl Default for App {
    fn default() -> Self {
        let mut editors = HashMap::new();

        editors.insert(Uuid::new_v4(), Editor::new(Rect::new(500.0, 20.0, 400.0, 400.0), 16));
        editors.insert(Uuid::new_v4(), Editor::new(Rect::new(20.0, 20.0, 400.0, 400.0), 16));

        Self {
            theme: Theme::mocha(),
            font: None,
            editors,
            highlighter: Highlighter::new(),
            focused: None,
            move_target: None,
            prompt_focused: false,
            released: true,
            prompt: Prompt::new()
        }
    }
}

#[derive(Debug)]
pub enum Message {
    Edit(Uuid, EditorMessage),
    Focus(Option<Uuid>),
    Scroll(Uuid, Vec2),
    Pan(Vec2),
    MoveTarget(Option<Uuid>),
    FocusPrompt(bool),
    KeyComboDone,
    PromptEdit(EditorMessage)
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Focus(uuid) => self.focused = uuid,
            Message::Edit(uuid, edit) =>
                self.editors.get_mut(&uuid).unwrap().update(edit, &mut self.highlighter, &self.theme),
            Message::Scroll(uuid, offset) => self.editors.get_mut(&uuid).unwrap().scroll(offset),
            Message::Pan(delta) => {
                match self.move_target {
                    Some(target) => {
                        let editor = self.editors.get_mut(&target).unwrap();
                        editor.window = editor.window.offset(delta)
                    },
                    None => {
                        self.pan(delta)
                    }
                }
            }
            Message::MoveTarget(target) => {
                self.move_target = target
            }
            Message::FocusPrompt(focused) => {
                if self.released {
                    self.prompt_focused = focused;
                }
                self.released = false;
            }
            Message::KeyComboDone => {
                self.released = true;
            }
            Message::PromptEdit(msg) => {
                if let Some(update) =  self.prompt.update(msg) {
                    match update {
                        _ => {}
                    }
                    self.prompt_focused = false;
                }
            }
        }
    }

    fn pan(&mut self, delta: Vec2) {
        for (_, editor) in &mut self.editors {
            editor.window = editor.window.offset(delta);
        }
    }

    fn handle_input(&self, messages: &mut Vec<Message>) {
        if get_keys_down().contains(&KeyCode::K)
            && (get_keys_down().contains(&KeyCode::LeftControl)
            || get_keys_down().contains(&KeyCode::RightControl)) {
            messages.push(Message::FocusPrompt(!self.prompt_focused));
            if !self.prompt_focused {
                return;
            }
        } else {
            if !self.released {
                messages.push(Message::KeyComboDone);
            }
            if self.prompt_focused {
                for key in get_keys_pressed() {
                    messages.push(Message::PromptEdit(EditorMessage::Keypress(key)));
                }

                if let Some(character) = get_char_pressed() {
                    messages.push(Message::PromptEdit(EditorMessage::Char(character)));
                }

                return;
            }
        }

        let hovered_editor = self.editors.iter()
            .find(|(_, editor)|
                editor.window.contains(Vec2::from(mouse_position()))
                    || editor.titlebar().contains(Vec2::from(mouse_position()))
            );

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some((uuid, _)) = hovered_editor {
                messages.push(Message::Focus(Some(*uuid)))
            } else {
                messages.push(Message::Focus(None))
            }
        }

        if let Some((uuid, _)) = hovered_editor {
            let wheel = mouse_wheel();
            if wheel.1 != 0.0 {
                messages.push(Message::Scroll(*uuid, Vec2::from(wheel) * 0.25 * vec2(0.0, -1.0)));
            }
        }

        let delta = mouse_delta_position() * -vec2(screen_width(), screen_height()) / 2.0;

        if is_mouse_button_down(MouseButton::Left) && delta != Vec2::ZERO {
            messages.push(Message::Pan(delta));
        } else {
            let hovered_titlebar = self.editors.iter().find(|(_, editor)| editor.titlebar().contains(Vec2::from(mouse_position())));
            let target = hovered_titlebar.and_then(|(uuid, _)| Some(*uuid));

            if self.move_target != target {
                messages.push(Message::MoveTarget(target));
            }
        }

        if let Some(uuid) = self.focused {
            for key in get_keys_pressed() {
                messages.push(Message::Edit(uuid, EditorMessage::Keypress(key)));
            }

            if let Some(character) = get_char_pressed() {
                messages.push(Message::Edit(uuid, EditorMessage::Char(character)));
            }
        }

        clear_input_queue();
    }

    pub fn view(&self) -> Vec<Message> {
        let mut messages = vec![];

        clear_background(self.theme.base);
        set_default_camera();
        // draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width() - 160., 32., 32., WHITE);

        for (uuid, editor) in &self.editors {
            editor.view(&self.theme, self.font.as_ref(), self.focused == Some(*uuid));
        }

        if self.prompt_focused {
            self.prompt.view(&self.theme, self.font.as_ref())
        }

        self.handle_input(&mut messages);
        messages
    }
}