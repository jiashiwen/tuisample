use std::cell::RefCell;

use anyhow::Result;
use crossterm::event::Event;
use log::info;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders};

use crate::{
    accessors,
    components::{
        CommandBlocking, CommandInfo,
        // CommitList,
        Component, DrawableComponent, event_pump, EventState, visibility_blocking,
    },
    keys::SharedKeyConfig,
    setup_popups,
    // queue::{Action, InternalEvent, Queue},
    strings,
    ui::style::SharedTheme,
};
use crate::cmdbar::CommandBar;
use crate::components::LoginComponent;

pub struct T01 {
    visible: bool,
    theme: SharedTheme,
    login: LoginComponent,
    cmdbar: RefCell<CommandBar>,
    // queue: Queue,
    key_config: SharedKeyConfig,
}

impl T01 {
    accessors!(self, [login]);

    setup_popups!(
        self,
        [
            login
        ]
    );
    ///
    pub fn new(
        // queue: &Queue,
        theme: SharedTheme,
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            visible: false,
            login: LoginComponent::new(key_config.clone()),
            cmdbar: RefCell::new(CommandBar::new(
                theme.clone(),
                key_config.clone(),
            )),
            theme,
            key_config,
        }
    }

    ///
    pub fn update(&mut self) -> Result<()> {
        if self.is_visible() {}

        Ok(())
    }

    fn apply_stash(&mut self) {}

    fn drop_stash(&mut self) {}

    fn pop_stash(&mut self) {}

    fn inspect(&mut self) {}
}

impl DrawableComponent for T01 {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let inner = Block::default().title("t01").borders(Borders::ALL);
        f.render_widget(inner, rect);
        // self.login.draw(f, rect);
        self.draw_popups(f)?;
        Ok(())
    }
}

impl Component for T01 {
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
                strings::commands::stashlist_pop(&self.key_config),
                // selection_valid,
                true,
                true,
            ));
            out.push(CommandInfo::new(
                strings::commands::stashlist_apply(&self.key_config),
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
            //     if k == self.key_config.login {
            //         self.login.show();
            //         return Ok(EventState::Consumed);
            //     } else if k == self.key_config.stash_apply {
            //         self.apply_stash();
            //     } else if k == self.key_config.stash_drop {
            //         self.drop_stash();
            //     } else if k == self.key_config.stash_open {
            //         self.inspect();
            //     }
            // }
            if event_pump(ev, self.components_mut().as_mut_slice())?
                .is_consumed()
            {
                let msg = self.login.get_msg();
                if !msg.is_empty() {
                    // self.list.list_item_add(msg);
                    info!("msg is {:?}",msg);
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
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;
        self.update()?;
        Ok(())
    }
}
