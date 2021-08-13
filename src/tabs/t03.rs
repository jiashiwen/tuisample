use anyhow::Result;
use crossterm::event::Event;
use tui::widgets::{Block, Borders};

use crate::{accessors, components::{
    CommandBlocking, CommandInfo,
    // CommitList,
    Component, DrawableComponent, EventState, visibility_blocking,
},
            keys::SharedKeyConfig,
            setup_popups,
            // queue::{Action, InternalEvent, Queue},
            strings,
            ui::style::SharedTheme,
};
use crate::components::{event_pump, PopInputComponent};

// use asyncgit::{
//     CWD,
//     sync::{self, CommitId},
// };

pub struct T03 {
    // list: CommitList,
    visible: bool,
    popinput: PopInputComponent,
    // queue: Queue,
    theme: SharedTheme,
    key_config: SharedKeyConfig,
}

impl T03 {
    accessors!(
        self,
        [
         popinput
        ]
    );
    // setup_popups!(
    //     self,
    //     [
    //         popinput
    //     ]
    // );
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
            popinput: PopInputComponent::new(key_config.clone()),
            theme,
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

impl DrawableComponent for T03 {
    fn draw<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        rect: tui::layout::Rect,
    ) -> Result<()> {
        let inner = Block::default().title("t03").borders(Borders::ALL);
        f.render_widget(inner, rect);
        self.popinput.draw(f, rect);
        // self.draw_popups(f)?;
        Ok(())
    }
}

impl Component for T03 {
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking {
        if self.visible || force_all {
            // self.list.commands(out, force_all);

            // let selection_valid =
            //     self.list.selected_entry().is_some();
            out.push(CommandInfo::new(
                strings::commands::ignore_item(&self.key_config),
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
            if let Event::Key(k) = ev {
                if k == self.key_config.status_ignore_file {
                    self.popinput.show();
                    return Ok(EventState::Consumed);
                } else if k == self.key_config.stash_drop {
                    self.drop_stash();
                } else if k == self.key_config.stash_open {
                    self.inspect();
                }
            }
            if event_pump(ev, self.components_mut().as_mut_slice())?
                .is_consumed()
            {
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
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;
        self.update()?;
        Ok(())
    }
}
