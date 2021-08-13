use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use log::info;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use unicode_width::UnicodeWidthStr;

use crate::{
    components::{CommandBlocking, CommandInfo, Component, DrawableComponent, EventState, visibility_blocking},
    strings,
};
use crate::keys::SharedKeyConfig;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct SearchComponent {
    visible: bool,
    input_mode: InputMode,
    input: String,
    //字符串指针位置
    input_position: usize,
    cursor_position: usize,
    message: String,
    key_config: SharedKeyConfig,
}

impl DrawableComponent for SearchComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> anyhow::Result<()> {
        if self.is_visible() {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ]
                        .as_ref(),
                )
                .split(rect);

            let input = Paragraph::new(self.input.as_ref())
                .style(match self.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, rect);
            match self.input_mode {
                InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                    {}

                InputMode::Editing => {
                    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[1].x + self.cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        rect.y + 1,
                    )
                }
            }
        }
        Ok(())
    }
}

impl Component for SearchComponent {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking {
        if self.is_visible() || force_all {
            let mut normal = true;
            match self.input_mode {
                InputMode::Editing => {
                    normal = false;
                }
                _ => {}
            }
            out.push(
                CommandInfo::new(
                    strings::commands::search_input_enable(&self.key_config),
                    true,
                    normal,
                )
                    .order(1)
            );
        }
        visibility_blocking(self)
    }

    fn event(&mut self, ev: Event) -> anyhow::Result<EventState> {
        if self.is_visible() {
            if let Event::Key(key) = ev {
                match self.input_mode {
                    InputMode::Editing => {
                        match key.code {

                            // KeyCode::Char('\n') => {
                            KeyCode::Enter => {
                                // self.message.push(self.input.drain(..).collect());
                                self.message = self.input.clone();
                                self.clear();
                            }
                            KeyCode::Char(c) => {
                                if self.input.width() == self.cursor_position {
                                    self.input.push(c);
                                    self.input_position = self.input.len();
                                    self.cursor_position = self.input.width();
                                }

                                if self.input.width() > self.cursor_position {
                                    self.input.insert(self.input_position, c);
                                    self.cursor_right();
                                }
                            }
                            KeyCode::Backspace => {
                                if self.cursor_left() {
                                    self.input.remove(self.input_position);
                                };
                            }

                            KeyCode::Delete => {
                                if self.input.len() > self.input_position {
                                    self.input.remove(self.input_position);
                                }
                            }
                            KeyCode::Left => {
                                self.cursor_left();
                            }
                            KeyCode::Right => {
                                self.cursor_right();
                            }
                            KeyCode::Esc => {
                                self.clear();
                                self.input_mode = InputMode::Normal;
                            }
                            _ => { return Ok(EventState::NotConsumed); }
                        }
                        return Ok(EventState::Consumed);
                    }
                    InputMode::Normal => {
                        match key.code {
                            KeyCode::Char('s') => {
                                self.input_mode = InputMode::Editing;
                                return Ok(EventState::Consumed);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
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
        self.input_mode = InputMode::Normal;
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;
        Ok(())
    }
}

impl SearchComponent {
    pub fn new(
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            // visible: false,
            visible: false,
            input_mode: InputMode::Normal,
            input: "".to_string(),
            input_position: 0,
            cursor_position: 0,
            // message: vec![],
            message: "".to_string(),
            key_config,
        }
    }

    pub fn get_msg(&mut self) -> String {
        let mut str = "".to_string();
        if !self.message.is_empty() {
            str = self.message.clone();
            self.message.clear();
        }

        return str;
    }

    pub fn get_input_mode(&self) -> &InputMode {
        return &self.input_mode;
    }

    fn clear(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
        self.input_position = 0;
    }

    fn cursor_left(&mut self) -> bool {
        if self.cursor_position > 0 {
            let mut index = self.input_position - 1;
            while index > 0
                && !self.input.is_char_boundary(index) {
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
        if self.cursor_position < self.input.width() {
            let mut index = self.input_position.saturating_add(1);
            while index < self.input.len()
                && !self.input.is_char_boundary(index) {
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