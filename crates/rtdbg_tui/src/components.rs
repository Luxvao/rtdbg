use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
};

pub enum TextInputStatus {
    Done(String),
    Cancelled,
    Continue,
}

#[derive(Debug, Default)]
enum Mode {
    #[default]
    Normal,
    Prefix(char),
    Insert,
}

#[derive(Debug)]
pub struct TextInput {
    title: String,
    buffer: Vec<char>,
    cursor: usize,
}

#[derive(Debug)]
pub struct MultiLineTextInput {
    mode: Mode,
    scroll_y: u16,
    scroll_x: u16,
    cursor_x: usize,
    cursor_y: usize,
    buffer: Vec<Vec<char>>,

    ro: bool,
}

impl TextInput {
    pub fn new(title: String) -> TextInput {
        TextInput {
            title,
            buffer: Vec::new(),
            cursor: 0,
        }
    }

    pub fn update(&mut self, press: KeyCode) -> TextInputStatus {
        match press {
            KeyCode::Char(a) => {
                self.buffer.insert(self.cursor, a);
                self.cursor = self.cursor.saturating_add(1);
            }
            KeyCode::Left => self.cursor = self.cursor.saturating_sub(1),
            KeyCode::Right => self.cursor = self.cursor.saturating_add(1).min(self.buffer.len()),
            KeyCode::Backspace => {
                if self.buffer.len() != 0 && self.cursor != 0 {
                    self.buffer.remove(self.cursor - 1);
                }

                self.cursor = self.cursor.saturating_sub(1);
            }
            KeyCode::Enter => return TextInputStatus::Done(self.buffer.iter().collect()),
            KeyCode::Esc => return TextInputStatus::Cancelled,
            _ => (),
        }

        TextInputStatus::Continue
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let popup = Rect {
            x: area.width / 3,
            y: area.height / 2 - 2,
            width: area.width / 3,
            height: 4,
        };

        frame.render_widget(Clear, popup);

        let span1 = Span::raw(self.buffer[0..self.cursor].iter().collect::<String>());

        let span2 =
            Span::raw(self.buffer.get(self.cursor).unwrap_or(&' ').to_string()).underlined();

        let span3 = Span::raw(
            self.buffer
                .get(self.cursor + 1..)
                .map(|v| v.iter().collect::<String>())
                .unwrap_or_default(),
        );

        let text = Line::from_iter([span1, span2, span3]);

        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(&*self.title))
                .wrap(ratatui::widgets::Wrap { trim: false }),
            popup,
        );
    }
}

impl MultiLineTextInput {
    pub fn update(&mut self, key: KeyCode) -> bool {
        match key {
            // Normal mode, movement
            a if let Mode::Normal = self.mode => match a {
                KeyCode::Esc => return true,
                KeyCode::Tab => return true,

                KeyCode::Char('i') if !self.ro => self.mode = Mode::Insert,

                KeyCode::Char('g') => self.mode = Mode::Prefix('g'),

                KeyCode::Char('h') => {
                    self.normalise_x();

                    self.cursor_x = self.cursor_x.saturating_sub(1);

                    if self.cursor_x == 0 && self.cursor_y != 0 {
                        self.cursor_y = self.cursor_y.saturating_sub(1);
                        self.cursor_x = self.buffer[self.cursor_y].len();
                    }
                }
                KeyCode::Char('j') => {
                    self.cursor_y = self
                        .cursor_y
                        .saturating_add(1)
                        .min(self.buffer.len().saturating_sub(1));
                }
                KeyCode::Char('k') => {
                    self.cursor_y = self.cursor_y.saturating_sub(1);
                }
                KeyCode::Char('l') => {
                    self.normalise_x();

                    if self.cursor_x == self.buffer[self.cursor_y].len()
                        && self.cursor_y != self.buffer.len().saturating_sub(1)
                    {
                        self.cursor_x = 0;
                        self.cursor_y = self
                            .cursor_y
                            .saturating_add(1)
                            .min(self.buffer.len().saturating_sub(1));

                        return false;
                    }

                    self.cursor_x = self.cursor_x.saturating_add(1);
                }
                _ => (),
            },

            // Prefix mode
            a if let Mode::Prefix('g') = self.mode => {
                self.mode = Mode::Normal;

                match a {
                    KeyCode::Char('g') => {
                        self.cursor_x = 0;
                        self.cursor_y = 0;
                    }
                    KeyCode::Char('e') => {
                        self.cursor_x = 0;
                        self.cursor_y = self.buffer.len().saturating_sub(1);
                    }
                    KeyCode::Char('h') => {
                        self.cursor_x = 0;
                    }
                    KeyCode::Char('l') => {
                        self.cursor_x = self.buffer[self.cursor_y].len();
                    }
                    _ => (),
                }
            }

            // Insert mode
            KeyCode::Esc => self.mode = Mode::Normal,
            KeyCode::Backspace => {
                self.normalise_x();

                if self.cursor_x == 0 {
                    let removed_line = self.buffer.remove(self.cursor_y);

                    self.cursor_x = if self.cursor_y == 0 {
                        self.cursor_y = self.cursor_y.saturating_sub(1);

                        0
                    } else {
                        self.cursor_y = self.cursor_y.saturating_sub(1);

                        self.buffer[self.cursor_y].len()
                    };

                    if self.buffer.is_empty() {
                        self.buffer.push(Vec::new());
                    }

                    self.buffer[self.cursor_y].extend(removed_line);

                    return false;
                }

                self.cursor_x = self.cursor_x.saturating_sub(1);
                self.buffer[self.cursor_y].remove(self.cursor_x);
            }
            KeyCode::Enter => {
                self.normalise_x();

                let removed = self.buffer[self.cursor_y].split_off(self.cursor_x);

                self.cursor_y = self.cursor_y.saturating_add(1);

                self.cursor_x = 0;

                self.buffer.insert(self.cursor_y, removed);
            }
            KeyCode::Tab => {
                self.normalise_x();

                for _ in 0..4 {
                    self.buffer[self.cursor_y].insert(self.cursor_x, ' ');
                    self.cursor_x = self.cursor_x.saturating_add(1);
                }
            }
            KeyCode::Char(a) => {
                self.normalise_x();

                self.buffer[self.cursor_y].insert(self.cursor_x, a);
                self.cursor_x = self.cursor_x.saturating_add(1);
            }

            _ => (),
        }

        false
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut text = Vec::new();

        for (i, line) in self.buffer.iter().enumerate() {
            if self.cursor_y == i {
                let span1 = Span::raw(
                    line[0..self.cursor_x.min(line.len())]
                        .iter()
                        .collect::<String>(),
                );

                let span2 = Span::raw(line.get(self.cursor_x).unwrap_or(&' ').to_string());

                let span2_styled = match self.mode {
                    Mode::Normal => span2.on_white().black(),
                    Mode::Insert => span2.underlined(),
                    _ => span2.on_white().black(),
                };

                let span3 = Span::raw(
                    line.get(self.cursor_x + 1..)
                        .map(|v| v.iter().collect::<String>())
                        .unwrap_or_default(),
                );

                let line = Line::from_iter([span1, span2_styled, span3]);

                text.push(line);

                continue;
            }

            text.push(Line::raw(line.iter().collect::<String>()));
        }

        let height = area.height;
        let width = area.width;

        if self.cursor_y <= self.scroll_y as usize {
            self.scroll_y = self.cursor_y as u16;
        } else if self.cursor_y >= self.scroll_y as usize + height as usize {
            self.scroll_y = self.cursor_y as u16 - height + 1;
        }

        if self.cursor_x <= self.scroll_x as usize {
            self.scroll_x = self.cursor_x as u16;
        } else if self.cursor_x >= self.scroll_x as usize + width as usize {
            self.scroll_x = self.cursor_x as u16 - width + 1;
        }

        let paragraph = Paragraph::new(text).scroll((self.scroll_y, self.scroll_x));

        frame.render_widget(paragraph, area);
    }

    pub fn set_text(&mut self, new: String) {
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.buffer = new
            .lines()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
    }

    pub fn get_text(&self) -> String {
        let mut text = String::new();

        for line in self.buffer.iter() {
            text.extend(line);
            text.push('\n');
        }

        text
    }

    pub fn read_only(&mut self, ro: bool) {
        self.ro = ro;
    }

    fn normalise_x(&mut self) {
        self.cursor_x = self.cursor_x.min(self.buffer[self.cursor_y].len());
    }
}

impl Default for MultiLineTextInput {
    fn default() -> Self {
        Self {
            mode: Default::default(),
            scroll_y: Default::default(),
            scroll_x: Default::default(),
            cursor_x: Default::default(),
            cursor_y: Default::default(),
            buffer: vec![vec![]],
            ro: false,
        }
    }
}

pub fn render_error(error: &str, frame: &mut Frame, area: Rect) {
    let popup = Rect {
        x: area.width / 3,
        y: area.height / 2 - 2,
        width: area.width / 3,
        height: 4,
    };

    frame.render_widget(Clear, popup);

    frame.render_widget(
        Paragraph::new(error)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(Block::bordered().title("Error")),
        popup,
    );
}

pub fn update_error(event: KeyCode) -> bool {
    if matches!(event, KeyCode::Enter | KeyCode::Esc) {
        return true;
    }

    false
}
