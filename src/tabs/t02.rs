use anyhow::Result;
use crossterm::event::Event;
use serde::__private::ser::constrain;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};

use crate::{
    components::{
        CommandBlocking,
        CommandInfo,
        Component,
        // CommitList,
        DrawableComponent, EventState, SearchComponent, visibility_blocking,
    },
    keys::SharedKeyConfig,
    // queue::{Action, InternalEvent, Queue},
    strings,
    ui::style::SharedTheme,
};

// use asyncgit::{
//     CWD,
//     sync::{self, CommitId},
// };

pub struct T02 {
    // list: CommitList,
    visible: bool,
    // queue: Queue,
    search: SearchComponent,
    key_config: SharedKeyConfig,
}

impl T02 {
    ///
    pub fn new(
        // queue: &Queue,
        theme: SharedTheme,
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            visible: false,
            // list: CommitList::new(
            //     &strings::stashlist_title(&key_config),
            //     theme,
            //     key_config.clone(),
            // ),
            // queue: queue.clone(),
            search: SearchComponent::new(key_config.clone()),
            key_config,
        }
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
        }

        Ok(())
    }

    fn apply_stash(&mut self) {
        // if let Some(e) = self.list.selected_entry() {
        //     match sync::stash_apply(CWD, e.id, false) {
        //         Ok(_) => {
        //             self.queue.push(InternalEvent::TabSwitch);
        //         }
        //         Err(e) => {
        //             self.queue.push(InternalEvent::ShowErrorMsg(
        //                 format!("stash apply error:\n{}", e, ),
        //             ));
        //         }
        //     }
        // }
    }

    fn drop_stash(&mut self) {
        // if let Some(e) = self.list.selected_entry() {
        //     self.queue.push(InternalEvent::ConfirmAction(
        //         Action::StashDrop(e.id),
        //     ));
        // }
    }

    fn pop_stash(&mut self) {
        // if let Some(e) = self.list.selected_entry() {
        //     self.queue.push(InternalEvent::ConfirmAction(
        //         Action::StashPop(e.id),
        //     ));
        // }
    }

    fn inspect(&mut self) {
        // if let Some(e) = self.list.selected_entry() {
        //     self.queue.push(InternalEvent::InspectCommit(e.id, None));
        // }
    }

    // Called when a pending stash action has been confirmed
    // pub fn action_confirmed(&self, action: &Action) -> bool {
    //     match *action {
    //         Action::StashDrop(id) => Self::drop(id),
    //         Action::StashPop(id) => self.pop(id),
    //         _ => false,
    //     }
    //
    // }

    // fn drop(id: CommitId) -> bool {
    //     sync::stash_drop(CWD, id).is_ok()
    // }

    // fn pop(&self, id: CommitId) -> bool {
    //     match sync::stash_pop(CWD, id) {
    //         Ok(_) => {
    //             self.queue.push(InternalEvent::TabSwitch);
    //             true
    //         }
    //         Err(e) => {
    //             self.queue.push(InternalEvent::ShowErrorMsg(
    //                 format!("stash pop error:\n{}", e, ),
    //             ));
    //             true
    //         }
    //     }
    // }
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

            self.search.draw(f, chunks[0])?;
            let inner = Block::default().title("t02").borders(Borders::ALL);
            f.render_widget(inner, chunks[1]);
            // self.list.draw(f, rect)?;
            //
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
            // if self.list.event(ev)?.is_consumed() {
            //     return Ok(EventState::Consumed);
            // }
            return self.search.event(ev);
            // if let Event::Key(k) = ev {
            //     if k == self.key_config.enter {
            //         self.pop_stash();
            //     } else if k == self.key_config.stash_apply {
            //         self.apply_stash();
            //     } else if k == self.key_config.stash_drop {
            //         self.drop_stash();
            //     } else if k == self.key_config.stash_open {
            //         self.inspect();
            //     }
            // }
        }

        Ok(EventState::NotConsumed)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false;
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;
        self.update()?;
        Ok(())
    }
}
