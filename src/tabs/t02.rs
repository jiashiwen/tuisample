use std::cell::Cell;

use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use log::info;
use serde::__private::ser::constrain;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};

pub use crate::{
    accessors,
    components::{
        CommandBlocking,
        CommandInfo,
        Component,
        // CommitList,
        DrawableComponent, event_pump, EventState, InputMode, SearchComponent, visibility_blocking,
    },
    keys::SharedKeyConfig,
    // queue::{Action, InternalEvent, Queue},

    ui::style::SharedTheme,
};
use crate::components::ListComponent;
use crate::strings;

enum Focus {
    List1,
    List2,
}

pub struct T02 {
    // list: CommitList,
    visible: bool,
    // queue: Queue,
    msg: Vec<String>,
    search: SearchComponent,
    list: ListComponent,
    list2: ListComponent,
    theme: SharedTheme,
    key_config: SharedKeyConfig,
}

impl T02 {
    accessors!(self, [list,list2,search]);
    ///
    pub fn new(
        // queue: &Queue,
        theme: SharedTheme,
        key_config: SharedKeyConfig,
    ) -> Self {
        let mut t02 =
            Self {
                visible: false,
                msg: vec![],
                search: SearchComponent::new(key_config.clone()),
                list: ListComponent::new_with_title("list1".to_string(),
                                                    theme.clone(),
                                                    key_config.clone()),
                list2: ListComponent::new_with_title("list2".to_string(),
                                                     theme.clone(),
                                                     key_config.clone()),
                theme,
                key_config,
            };
        t02.list.focus(true);
        return t02;
    }


    ///
    pub fn update(&mut self) -> Result<()> {
        if self.is_visible() {
            // let stashes = sync::get_stashes(CWD)?;
            // let commits =
            //     sync::get_commits_info(CWD, stashes.as_slice(), 100)?;
            //
            // self.list.set_count_total(commits.len());
            // self.list.items().set_items(0, commits);
            // info!("{:?}",self.get_search_msg());
        }

        Ok(())
    }

    fn toggle_focus(&mut self) {
        if self.list.focused() {
            self.list.focus(false);
            self.list2.focus(true);
            return;
        }
        if self.list2.focused() {
            self.list2.focus(false);
            self.list.focus(true);
            return;
        }
    }
}

impl DrawableComponent for T02 {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
    ) -> Result<()> {
        if self.is_visible() {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(9)
                    ]
                ).split(rect);
            let list_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(70)
                    ]
                ).split(chunks[1]);

            self.search.draw(f, chunks[0])?;
            self.list.draw(f, list_chunk[0])?;
            self.list2.draw(f, list_chunk[1])?;
        }
        Ok(())
    }
}


impl Component for T02 {
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking {
        if self.visible || force_all {
            // self.list.commands(out, force_all);
            self.search.commands(out, force_all);

            // let selection_valid =
            //     self.list.selected_entry().is_some();
            // out.push(CommandInfo::new(
            //     strings::commands::stashing_save(&self.key_config),
            //     // selection_valid,
            //     true,
            //     true,
            // ));
            out.push(CommandInfo::new(
                strings::commands::log_details_open(&self.key_config),
                // selection_valid,
                true,
                true,
            ));
            out.push(CommandInfo::new(
                strings::commands::stashlist_drop(&self.key_config),
                // selection_valid,
                true,
                true,
            ));
            out.push(CommandInfo::new(
                strings::commands::stashlist_inspect(
                    &self.key_config,
                ),
                // selection_valid,
                true,
                true,
            ));
        }

        visibility_blocking(self)
    }

    fn event(
        &mut self,
        ev: crossterm::event::Event,
    ) -> Result<EventState> {
        if self.is_visible() {
            // if let Event::Key(k) = ev {
            //     if k == self.key_config.focus_left {
            //         self.toggle_focus();
            //         return Ok(EventState::Consumed);
            //     }
            //     if k == self.key_config.focus_right {
            //         self.toggle_focus();
            //         return Ok(EventState::Consumed);
            //     }
            // }
            match self.search.get_input_mode() {
                InputMode::Normal => {
                    self.list.enable_event();
                    self.list2.enable_event();

                    if let Event::Key(k) = ev {
                        if k == self.key_config.focus_left {
                            self.toggle_focus();
                            return Ok(EventState::Consumed);
                        }
                        if k == self.key_config.focus_right {
                            self.toggle_focus();
                            return Ok(EventState::Consumed);
                        }
                    }
                }
                _ => {}
            }
            if event_pump(ev, self.components_mut().as_mut_slice())?
                .is_consumed()
            {
                match self.search.get_input_mode() {
                    InputMode::Editing => {
                        self.list.unselected();
                        self.list.disable_event();
                        self.list2.unselected();
                        self.list2.disable_event();
                    }
                    _ => {}
                }
                let msg = self.search.get_msg();
                if !msg.is_empty() {
                    if self.list.focused() {
                        self.list.list_item_add(msg);
                    } else {
                        self.list2.list_item_add(msg);
                    }
                }
                return Ok(EventState::Consumed);
            }
        }

        Ok(EventState::NotConsumed)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false;
        self.search.hide();
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;
        self.search.show();
        self.update()?;
        Ok(())
    }
}
