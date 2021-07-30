use crossterm::event::Event;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::text::Text;
use tui::widgets::{Block, Borders, List, ListItem};

use crate::components::{CommandBlocking, CommandInfo, Component, DrawableComponent, EventState};
use crate::keys::SharedKeyConfig;

pub struct ListComponent {
    list_items: Vec<String>,
    key_config: SharedKeyConfig,
}

impl Component for ListComponent {
    fn commands(&self, out: &mut Vec<CommandInfo>, force_all: bool) -> CommandBlocking {
        todo!()
    }

    fn event(&mut self, ev: Event) -> anyhow::Result<EventState> {
        todo!()
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
                .block(Block::default().borders(Borders::ALL).title("List"));
            f.render_widget(msglist, rect);
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
            key_config,
        }
    }

    pub fn list_item_add(&mut self, str: String) {
        self.list_items.push(str);
    }
}