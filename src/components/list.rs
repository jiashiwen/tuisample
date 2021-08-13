use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};

use crossterm::event::{Event, KeyCode};
use log::info;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, Clear, List, ListItem, ListState};

use crate::components::{CommandBlocking, CommandInfo, Component, DrawableComponent, EventState};
use crate::keys::SharedKeyConfig;
use crate::ui::style::SharedTheme;

pub struct ListComponent {
    title: String,
    selected: bool,
    theme: SharedTheme,
    event_enable: bool,
    list_items: Vec<String>,
    state: RefCell<ListState>,
    key_config: SharedKeyConfig,
}

impl Component for ListComponent {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking {
        todo!()
    }

    fn event(&mut self, ev: Event) -> anyhow::Result<EventState> {
        if self.is_visible() && self.event_enable && self.selected {
            if let Event::Key(key) = ev {
                match key.code {
                    KeyCode::Down => {
                        self.next();
                        return Ok(EventState::Consumed);
                    }
                    KeyCode::Up => {
                        self.previous();
                        return Ok(EventState::Consumed);
                    }
                    _ => {}
                }

                if key == self.key_config.stash_drop {
                    self.remove_line();
                }
            }
        }
        Ok(EventState::NotConsumed)
    }

    fn focused(&self) -> bool {
        self.selected
    }

    fn focus(&mut self, _focus: bool) {
        self.selected = _focus;
    }
}

impl DrawableComponent for ListComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> anyhow::Result<()> {
        if self.is_visible() {
            let list_items: Vec<ListItem> = self.list_items.iter().
                map(|i| {
                    ListItem::new(Text::raw(i))
                })
                .collect();

            let msglist = List::new(list_items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title(self.title.clone())
                    .border_style(self.theme.block(self.selected)))
                .highlight_style(
                    // Style::default()
                    self.theme.commit_author(true)
                    // .add_modifier(Modifier::UNDERLINED),
                );
            let mut stat = self.state.borrow_mut();
            f.render_stateful_widget(msglist, rect, &mut stat);
            // f.render_widget(msglist, rect);
        }
        Ok(())
    }
}

impl ListComponent {
    pub fn new(
        theme: SharedTheme,
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            title: "".to_string(),
            selected: false,
            theme,
            event_enable: true,
            list_items: vec!["1".to_string(), "2".to_string()],
            state: RefCell::new(ListState::default()),
            key_config,
        }
    }

    pub fn new_with_title(
        title: String,
        theme: SharedTheme,
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            title,
            selected: false,
            theme,
            event_enable: true,
            list_items: vec!["1".to_string(), "2".to_string()],
            state: RefCell::new(ListState::default()),
            key_config,
        }
    }

    pub fn enable_event(&mut self) {
        self.event_enable = true;
    }

    pub fn disable_event(&mut self) {
        self.event_enable = false;
    }
    pub fn next(&mut self) {
        let i = match self.state.get_mut().selected() {
            Some(i) => {
                if i >= self.list_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.get_mut().select(Some(i));
        info!("selected {}",i);
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }


    pub fn previous(&mut self) {
        let i = match self.state.get_mut().selected() {
            Some(i) => {
                if i == 0 {
                    self.list_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.get_mut().select(Some(i));
    }

    pub fn unselected(&mut self) {
        self.state.get_mut().select(None);
    }

    fn remove_line(&mut self) {
        match self.state.get_mut().selected() {
            Some(idx) => {
                self.list_items.swap_remove(idx);
                info!("idx is {}",idx);
                info!("items length {}",self.list_items.len());
                if idx >= self.list_items.len() {
                    if self.list_items.len() == 0 {
                        self.unselected();
                        return;
                    }
                    self.previous();
                }
            }
            None => {}
        };
    }
    pub fn list_item_add(&mut self, str: String) {
        self.list_items.push(str);
    }
}