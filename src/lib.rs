use std::collections::HashMap;
use inkjet::Highlighter;
use macroquad::prelude::*;
use uuid::Uuid;
use crate::editor::{Editor, EditorMessage};
use crate::theme::Theme;

pub mod theme;
mod editor;
mod window;

pub struct App {
    theme: Theme,
    pub font: Option<Font>,
    editors: HashMap<Uuid, Editor>,
    highlighter: Highlighter,
    focused: Option<Uuid>,
    mouse_pan: Option<Vec2>
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
            mouse_pan: None,
        }
    }
}

#[derive(Debug)]
pub enum Message {
    Edit(Uuid, EditorMessage),
    Focus(Option<Uuid>),
    Scroll(Uuid, Vec2),
    Pan(Vec2),
    StopPan,
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Focus(uuid) => self.focused = uuid,
            Message::Edit(uuid, edit) =>
                self.editors.get_mut(&uuid).unwrap().update(edit, &mut self.highlighter, &self.theme),
            Message::Scroll(uuid, offset) => self.editors.get_mut(&uuid).unwrap().scroll(offset),
            Message::Pan(mouse) => {
                if let Some(prev_mouse) = self.mouse_pan {
                    self.pan(mouse - prev_mouse);
                }
                self.mouse_pan = Some(mouse);
            }
            Message::StopPan => {
                self.mouse_pan = None;
            }
        }
    }

    fn pan(&mut self, delta: Vec2) {
        for (_, editor) in &mut self.editors {
            editor.window = editor.window.offset(delta);
        }
    }

    fn handle_input(&self, messages: &mut Vec<Message>) {
        let hovered_editor = self.editors.iter()
            .find(|(_, editor)| editor.window.contains(Vec2::from(mouse_position())));

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

        if hovered_editor.is_none() && is_mouse_button_down(MouseButton::Left) {
            messages.push(Message::Pan(Vec2::from(mouse_position())));
        } else if self.mouse_pan.is_some() {
            messages.push(Message::StopPan);
        }

        if let Some(uuid) = self.focused {
            for key in get_keys_pressed() {
                messages.push(Message::Edit(uuid, EditorMessage::Keypress(key)));
            }

            if let Some(character) = get_char_pressed() {
                messages.push(Message::Edit(uuid, EditorMessage::Char(character)));
            }
        }
    }

    pub fn view(&self) -> Vec<Message> {
        let mut messages = vec![];

        clear_background(self.theme.base);
        set_default_camera();
        // draw_text(format!("FPS: {}", get_fps()).as_str(), screen_width() - 160., 32., 32., WHITE);

        for (uuid, editor) in &self.editors {
            editor.view(&self.theme, self.font.as_ref(), self.focused == Some(*uuid));
        }

        self.handle_input(&mut messages);
        messages
    }
}