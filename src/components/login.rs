use std::collections::HashMap;

use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use log::info;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use unicode_width::UnicodeWidthStr;

use crate::{components::{CommandBlocking, CommandInfo, Component, DrawableComponent, EventState, visibility_blocking}, strings, ui};
use crate::keys::SharedKeyConfig;

enum Focus {
    UserName,
    Password,
}

pub struct Input {
    text: String,
    showtext: String,
    input_position: usize,
    cursor_position: usize,
}

impl Input {
    pub fn new() -> Self {
        Input {
            text: "".to_string(),
            showtext: "".to_string(),
            input_position: 0,
            cursor_position: 0,
        }
    }

    fn clear(&mut self) {
        self.text.clear();
        self.showtext.clear();
        self.cursor_position = 0;
        self.input_position = 0;
    }

    fn cursor_left(&mut self) -> bool {
        if self.cursor_position > 0 {
            let mut index = self.input_position - 1;
            while index > 0
                && !self.text.is_char_boundary(index) {
                index -= 1;
            }
            if self.input_position - index > 1 {
                self.cursor_position -= 2;
            } else {
                self.cursor_position -= 1;
            }
            self.input_position = index;

            return true;
        }
        return false;
    }

    fn cursor_right(&mut self) {
        if self.cursor_position < self.text.width() {
            let mut index = self.input_position.saturating_add(1);
            while index < self.text.len()
                && !self.text.is_char_boundary(index) {
                index += 1;
            }
            if index - self.input_position > 1 {
                self.cursor_position += 2;
            } else {
                self.cursor_position += 1;
            }
            self.input_position = index;
        }
    }
}

pub struct LoginComponent {
    visible: bool,
    focus: Focus,
    username: Input,
    password: Input,
    message: HashMap<String, String>,
    key_config: SharedKeyConfig,
}

impl DrawableComponent for LoginComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        _rect: Rect,
    ) -> anyhow::Result<()> {
        if self.is_visible() {
            const SIZE: (u16, u16) = (40, 8);
            let area = ui::centered_rect_absolute(SIZE.0, SIZE.1, f.size());

            let userinput = Paragraph::new(self.username.text.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("UserName"));
            let passwordinput = Paragraph::new(self.password.showtext.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Password"));

            let login_block = Block::default().title("Login").borders(Borders::ALL);
            f.render_widget(Clear, area);
            f.render_widget(login_block, area);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                        .as_ref(),
                )
                .split(area);

            f.render_widget(userinput, chunks[0]);
            f.render_widget(passwordinput, chunks[1]);

            match self.focus {
                Focus::UserName => {
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[0].x + self.username.cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[0].y + 1,
                    )
                }

                Focus::Password => {
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[1].x + self.password.cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[1].y + 1,
                    )
                }
            }
        }
        Ok(())
    }
}

impl Component for LoginComponent {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking {
        if self.is_visible() || force_all {}
        visibility_blocking(self)
    }

    fn event(&mut self, ev: Event) -> anyhow::Result<EventState> {
        if self.is_visible() {
            if let Event::Key(key) = ev {
                match key.code {
                    KeyCode::Enter => {
                        self.message.clear();
                        if !self.username.text.is_empty() {
                            self.message.insert("username".to_string(), self.username.text.clone());
                            self.message.insert("password".to_string(), self.password.text.clone());
                            self.clear();
                        }
                    }
                    KeyCode::Char(c) => {
                        match self.focus {
                            Focus::UserName => {
                                if self.username.text.width() == self.username.cursor_position {
                                    self.username.text.push(c);
                                    self.username.input_position = self.username.text.len();
                                    self.username.cursor_position = self.username.text.width();
                                }

                                if self.username.text.width() > self.username.cursor_position {
                                    self.username.text.insert(self.username.input_position, c);
                                    self.username.cursor_right();
                                }
                            }
                            Focus::Password => {
                                if self.password.text.width() == self.password.cursor_position {
                                    self.password.text.push(c);
                                    self.password.input_position = self.password.text.len();
                                    self.password.cursor_position = self.password.text.width();
                                    self.password.showtext.push('*');
                                }

                                if self.password.text.width() > self.password.cursor_position {
                                    self.password.text.insert(self.password.input_position, c);
                                    self.password.cursor_right();
                                    self.password.showtext.push('*');
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        match self.focus {
                            Focus::UserName => {
                                if self.username.cursor_left() {
                                    self.username.text.remove(self.username.input_position);
                                };
                            }
                            Focus::Password => {
                                if self.password.cursor_left() {
                                    self.password.text.remove(self.password.input_position);
                                    self.password.showtext.pop();
                                };
                            }
                        }
                    }

                    KeyCode::Delete => {
                        match self.focus {
                            Focus::UserName => {
                                if self.username.text.len() > self.username.input_position {
                                    self.username.text.remove(self.username.input_position);
                                }
                            }
                            Focus::Password => {
                                if self.password.text.len() > self.password.input_position {
                                    self.password.text.remove(self.password.input_position);
                                    self.password.showtext.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Left => {
                        match self.focus {
                            Focus::UserName => {
                                self.username.cursor_left();
                            }
                            Focus::Password => {
                                self.password.cursor_left();
                            }
                        }
                    }
                    KeyCode::Right => {
                        match self.focus {
                            Focus::UserName => {
                                self.username.cursor_right();
                            }
                            Focus::Password => {
                                self.password.cursor_right();
                            }
                        }
                    }
                    KeyCode::Up => {
                        self.toggle_focus();
                    }
                    KeyCode::Down => {
                        self.toggle_focus();
                    }
                    KeyCode::Tab => {}
                    KeyCode::Esc => {
                        self.clear();
                        self.hide();
                    }
                    _ => { return Ok(EventState::NotConsumed); }
                }
                return Ok(EventState::Consumed);
            }
        } else {
            if let Event::Key(key) = ev {
                if key == self.key_config.login {
                    self.show()?;
                    return Ok(EventState::Consumed);
                }
            }
        }


        Ok(EventState::NotConsumed)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.clear();
        self.visible = false;
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;
        Ok(())
    }
}

impl LoginComponent {
    pub fn new(
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            visible: false,
            focus: Focus::UserName,
            username: Input::new(),
            password: Input::new(),
            message: HashMap::new(),
            key_config,
        }
    }

    pub fn get_msg(&mut self) -> HashMap<String, String> {
        let mut hash = HashMap::new();
        if !self.message.is_empty() {
            hash = self.message.clone();
            self.message.clear();
        }

        return hash;
    }

    fn clear(&mut self) {
        self.username.clear();
        self.password.clear();
        self.focus = Focus::UserName;
    }

    fn toggle_focus(&mut self) {
        match self.focus {
            Focus::UserName => {
                self.focus = Focus::Password
            }
            Focus::Password => {
                self.focus = Focus::UserName
            }
        }
    }
}