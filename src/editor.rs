use macroquad::prelude::*;
use crate::theme::Theme;
use crate::window::{set_camera_window, set_fullscreen_camera};
use inkjet::{Highlighter, Language};
use inkjet::constants::HIGHLIGHT_NAMES;
use inkjet::theme::vendored;
use inkjet::tree_sitter_highlight::HighlightEvent;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Point {
    row: usize,
    column: usize,
}

impl Point {
    pub fn new(row: usize, column: usize) -> Self {
        Point { row, column }
    }
}

#[derive(Debug)]
struct ColorSpan {
    start: Point,
    end: Point,
    color: Color,
}

pub(crate) struct Editor {
    lines: Vec<String>,
    cursor_position: Point, // line, character
    colors: Vec<ColorSpan>,
    pub(crate) window: Rect,
    font_size: u16,
    offset: Vec2
}

pub enum EditorMessage {
    Keypress(KeyCode),
    Char(char),
}

impl Editor {
    pub fn new(window: Rect, font_size: u16) -> Self {
        Self {
            lines: vec!["".to_owned()],
            cursor_position: Point::new(0, 0),
            colors: vec![],
            window,
            font_size,
            offset: Vec2::ZERO
        }
    }

    fn move_cursor(&mut self, key: KeyCode) {
        use KeyCode::*;
        match key {
            Up => {
                if self.cursor_position.row > 0 {
                    self.cursor_position.row -= 1
                }
            }
            Left => {
                if self.cursor_position.column > 0 {
                    self.cursor_position.column -= 1
                }
            }
            Right => self.cursor_position.column += 1,
            Down => self.cursor_position.row += 1,
            _ => {}
        }

        if self.cursor_position.row >= self.lines.len() {
            self.cursor_position.row = self.lines.len() - 1
        }

        if self.cursor_position.column >= self.lines[self.cursor_position.row].len() {
            self.cursor_position.column = self.lines[self.cursor_position.row].len()
        }
    }

    fn insert_str_single_line(&mut self, string: &str) {
        self.lines[self.cursor_position.row].insert_str(self.cursor_position.column, string);
        self.cursor_position.column += string.len();
    }

    pub fn scroll(&mut self, offset: Vec2) {
        self.offset += offset;

        if self.offset.y > self.lines.len() as f32 * self.font_size as f32 - self.window.h {
            self.offset.y = self.lines.len() as f32 * self.font_size as f32 - self.window.h;
        }


        if self.offset.y < 0.0 {
            self.offset.y = 0.0;
        }

    }

    pub fn update(&mut self, message: EditorMessage, highlighter: &mut Highlighter, theme: &Theme) {
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
                let Point { row: y, column: x } = self.cursor_position;

                match character {
                    '\x08' => {
                        if x > 0 {
                            self.lines[y].remove(x - 1);
                            self.cursor_position.column -= 1;
                        } else if y > 0 {
                            let original_line = &self.lines[y].clone();
                            let restored_position = self.lines[y - 1].len();
                            self.lines[y - 1] += original_line;
                            self.lines.remove(y);
                            self.cursor_position = Point::new(y - 1, restored_position);
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
                        self.cursor_position = Point::new(y + 1, 0);
                    }
                    _ => {
                        self.lines[y].insert(x, character);
                        self.cursor_position.column += 1;
                    }
                }
            }
        }

        if let Err(_) = self.syntax_highlight(highlighter, theme) {
            self.colors = vec![]
        }

        let effective_height = self.cursor_position.row as f32 * self.font_size as f32 - self.offset.y;


        if effective_height < 0.0 {
            self.offset.y += effective_height;
        }

        if effective_height > self.window.h {
            self.offset.y += effective_height - (self.window.h - self.font_size as f32);
        }
    }

    fn idx_to_point(code: &str, idx: usize) -> Point {
        let slice = &code[..idx];
        let lines: Vec<&str> = slice.split("\n").collect();

        Point::new(lines.len() - 1, lines.last().unwrap().len())
    }

    fn syntax_highlight(&mut self, highlighter: &mut Highlighter, theme: &Theme) -> inkjet::Result<()> {
        let code = self.lines.join("\n");
        let highlights = highlighter.highlight_raw(Language::Python, &code)?;

        self.colors = vec![];

        let mut color = theme.text;

        for highlight in highlights {
            let highlight = highlight?;

            match highlight {
                HighlightEvent::HighlightStart(style) => {
                    let name = HIGHLIGHT_NAMES[style.0];
                    let inkjet_theme = inkjet::theme::Theme::from_helix(vendored::CATPPUCCIN_MOCHA)?;
                    if let Some(fg) = inkjet_theme.get_style(name).and_then(|s| s.fg) {
                        color = Color::from_rgba(fg.r, fg.g, fg.b, 255);
                    }
                }

                HighlightEvent::Source { start, end } => {
                    self.colors.push(ColorSpan {
                        start: Self::idx_to_point(&code, start),
                        end: Self::idx_to_point(&code, end),
                        color,
                    });
                }

                HighlightEvent::HighlightEnd => {
                    color = theme.text;
                }
            }
        }

        Ok(())
    }

    fn format_line_number(&self, i: usize) -> String {
        let width = self.lines.len().to_string().len();
        format!("{:>width$} ", i + 1)
    }

    pub fn view(&self, theme: &Theme, font: Option<&Font>, focused: bool) {
        set_fullscreen_camera();
        draw_rectangle(self.window.x, self.window.y, self.window.w, self.window.h, theme.surface0);

        set_camera_window(self.window, self.offset);

        let mut x = 0.0;
        let mut y = self.font_size as f32;

        for (i, line) in self.lines.iter().enumerate() {
            for symbol in self.format_line_number(i).chars() {
                let dimensions = draw_text_ex(&symbol.to_string(), x, y, TextParams {
                    color: theme.overlay1,
                    font,
                    font_size: self.font_size,
                    ..Default::default()
                });
                x += dimensions.width;
            }

            for (j, glyph) in line.chars().enumerate() {
                let curr = Point::new(i, j);

                if curr == self.cursor_position && focused {
                    draw_rectangle(x, y - self.font_size as f32, 2.0, self.font_size as f32, theme.text);
                }

                let span = self.colors.iter().find(|span| {
                    span.start <= curr && curr < span.end
                });

                let color = span.and_then(|span| Some(span.color)).unwrap_or(theme.text);

                let dimensions = draw_text_ex(&glyph.to_string(), x, y, TextParams {
                    color,
                    font,
                    font_size: self.font_size,
                    ..Default::default()
                });

                x += dimensions.width;
            }

            if Point::new(i, line.len()) == self.cursor_position && focused {
                draw_rectangle(x, y - self.font_size as f32, 2.0, self.font_size as f32, theme.text)
            }

            x = 0.0;
            y += self.font_size as f32;
        }

        set_default_camera();
    }
}