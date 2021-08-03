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

pub struct ListStateful<T> {
    pub stat: ListState,
    pub items: Vec<T>,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    // pub fn next(&mut self) {
    //     let i = match self.state.selected() {
    //         Some(i) => {
    //             if i >= self.items.len() - 1 {
    //                 0
    //             } else {
    //                 i + 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.state.select(Some(i));
    // }
    //
    // pub fn previous(&mut self) {
    //     let i = match self.state.selected() {
    //         Some(i) => {
    //             if i == 0 {
    //                 self.items.len() - 1
    //             } else {
    //                 i - 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.state.select(Some(i));
    // }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct ListComponent {
    list_items: Vec<String>,
    state: RefCell<ListState>,
    key_config: SharedKeyConfig,
}

impl Component for ListComponent {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking {
        todo!()
    }

    fn event(&mut self, ev: Event) -> anyhow::Result<EventState> {
        if self.is_visible() {
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
            }
        }
        Ok(EventState::NotConsumed)
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
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
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
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            list_items: vec!["1".to_string(), "2".to_string()],
            state: RefCell::new(ListState::default()),
            key_config,
        }
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
    pub fn list_item_add(&mut self, str: String) {
        self.list_items.push(str);
    }
}