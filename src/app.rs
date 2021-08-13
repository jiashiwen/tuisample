use std::cell::{Cell, RefCell};
use std::rc::Rc;

use anyhow::{bail, Result};
use crossterm::event::{Event, KeyEvent};
use log::info;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Margin, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};

use crate::{accessors, setup_popups};
use crate::cmdbar::CommandBar;
use crate::components::{CommandBlocking, CommandInfo, Component, DrawableComponent, event_pump, HelpComponent};
use crate::keys::{KeyConfig, SharedKeyConfig};
use crate::strings;
use crate::strings::order;
use crate::tabs::{T01, T02, T03};
use crate::ui::style::{SharedTheme, Theme};

// use crate::util::{RandomSignal, SinSignal, TabsState};

pub struct App {
    do_quit: bool,
    help: HelpComponent,
    t01: T01,
    t02: T02,
    t03: T03,
    pub should_quit: bool,
    pub enhanced_graphics: bool,
    pub key_config: SharedKeyConfig,
    cmdbar: RefCell<CommandBar>,
    theme: SharedTheme,
    pub tab: usize,
    requires_redraw: Cell<bool>,
    // queue: Queue,
}

// pub enum Event<I> {
//     Input(I),
//     Tick,
// }

impl App {
    #[allow(clippy::too_many_lines)]
    pub fn new(enhanced_graphics: bool, theme: Theme, key_config: KeyConfig) -> Self {
        let theme = Rc::new(theme);
        // let queue = Queue::new();
        let key_config = Rc::new(key_config);

        let mut app = App {
            // title: title,
            should_quit: false,
            key_config: key_config.clone(),
            do_quit: false,
            enhanced_graphics: enhanced_graphics,
            cmdbar: RefCell::new(CommandBar::new(
                theme.clone(),
                key_config.clone(),
            )),
            tab: 0,
            requires_redraw: Cell::new(false),
            t01: T01::new(
                // &queue,
                theme.clone(),
                key_config.clone(),
            ),
            t02: T02::new(
                // &queue,
                theme.clone(),
                key_config.clone(),
            ),
            t03: T03::new(
                // &queue,
                theme.clone(),
                key_config.clone(),
            ),
            help: HelpComponent::new(
                theme.clone(),
                key_config.clone(),
            ),
            theme,
            // queue,
        };
        app.set_tab(0);
        return app;
    }

    // pub fn on_up(&mut self) {
    //     self.tasks.previous();
    // }

    // pub fn on_down(&mut self) {
    //     self.tasks.next();
    // }


    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> Result<()> {
        let fsize = f.size();

        self.cmdbar.borrow_mut().refresh_width(fsize.width);

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Min(2),
                    Constraint::Length(self.cmdbar.borrow().height()),
                ]
                    .as_ref(),
            )
            .split(fsize);
        self.cmdbar.borrow().draw(f, chunks_main[2]);

        self.draw_tabs(f, chunks_main[0]);

        //TODO: macro because of generic draw call
        match self.tab {
            // match self.tabs.index {
            // 0 => self.status_tab.draw(f, chunks_main[1])?,
            // 1 => self.revlog.draw(f, chunks_main[1])?,
            // 2 => self.files_tab.draw(f, chunks_main[1])?,
            // 3 => self.stashing_tab.draw(f, chunks_main[1])?,
            // 4 => self.stashlist_tab.draw(f, chunks_main[1])?,
            // 0 => self.draw_first_tab(f, chunks_main[1]),
            // 1 => self.draw_second_tab(f, chunks_main[1]),
            0 => self.t01.draw(f, chunks_main[1])?,
            1 => self.t02.draw(f, chunks_main[1])?,
            2 => self.t03.draw(f, chunks_main[1])?,
            _ => bail!("unknown tab"),
        };

        self.draw_popups(f)?;

        Ok(())
    }

    fn draw_tabs<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        let tabs = [
            Span::raw(strings::tab_t01(&self.key_config)),
            Span::raw(strings::tab_t02(&self.key_config)),
            Span::raw(strings::tab_t03(&self.key_config))
        ]
            .iter()
            .cloned()
            .map(Spans::from)
            .collect();

        f.render_widget(
            Tabs::new(tabs)
                .block(
                    Block::default()
                        .borders(Borders::BOTTOM)
                        .border_style(self.theme.block(false)),
                )
                .style(self.theme.tab(false))
                .highlight_style(self.theme.tab(true))
                .divider(strings::tab_divider(&self.key_config))
                .select(self.tab),
            r,
        );
    }
    pub fn event(&mut self, ev: Event) -> Result<()> {
        if event_pump(ev, self.components_mut().as_mut_slice())?
            .is_consumed() {
            return Ok(());
        }

        if let Event::Key(k) = ev {
            if k == self.key_config.open_help {
                self.help.show();
            }
            if k == self.key_config.quit || k == self.key_config.exit {
                self.do_quit = true;
                return Ok(());
            }
            if k == self.key_config.tab_toggle {
                self.toggle_tabs(false);
                self.update();
                return Ok(());
            }

            if k == self.key_config.tab_status
                || k == self.key_config.tab_log
                || k == self.key_config.tab_files
            {
                self.switch_tab(k)?;
                self.update();
                return Ok(());
            }
        }

        Ok(())
    }


    pub fn update(&mut self) -> Result<()> {
        log::trace!("update");

        self.update_commands();

        Ok(())
    }

    pub fn update_commands(&mut self) {
        self.help.set_cmds(self.commands(true));
        self.cmdbar.borrow_mut().set_cmds(self.commands(false));
    }

    pub fn requires_redraw(&self) -> bool {
        if self.requires_redraw.get() {
            self.requires_redraw.set(false);
            true
        } else {
            false
        }
    }

    // pub fn on_right(&mut self) {
    //     if self.tab == 2 {
    //         self.tab = 0;
    //     } else {
    //         self.tab = self.tab + 1;
    //     }
    //     // self.tabs.next();
    // }

    // pub fn on_left(&mut self) {
    //     if self.tab == 0 {
    //         self.tab = 2;
    //     } else {
    //         self.tab = self.tab - 1;
    //     }
    //     // self.tabs.previous();
    // }

    // pub fn on_key(&mut self, c: char) {
    //     match c {
    //         'q' => {
    //             self.should_quit = true;
    //         }
    //         // 't' => {
    //         //     self.show_chart = !self.show_chart;
    //         // }
    //         _ => {}
    //     }
    // }

    pub const fn is_quit(&self) -> bool {
        self.do_quit
    }

    fn check_hard_exit(&mut self, ev: Event) -> bool {
        if let Event::Key(e) = ev {
            if e == self.key_config.exit {
                self.do_quit = true;
                return true;
            }
        }
        false
    }

    pub fn on_tick(&mut self) {
        self.update();
        // Update progress
        // self.progress += 0.001;
        // if self.progress > 1.0 {
        //     self.progress = 0.0;
        // }
        //
        // self.sparkline.on_tick();
        // self.signals.on_tick();
        //
        // let log = self.logs.items.pop().unwrap();
        // self.logs.items.insert(0, log);
        //
        // let event = self.barchart.pop().unwrap();
        // self.barchart.insert(0, event);
    }
}

//private
impl App {
    accessors!(
        self,
        [
            t01,
            t02,
            t03,
            help
        ]
    );

    setup_popups!(
        self,
        [
            help
        ]
    );

    fn toggle_tabs(&mut self, reverse: bool) -> Result<()> {
        let tabs_len = self.get_tabs().len();
        let new_tab = if reverse {
            self.tab.wrapping_sub(1).min(tabs_len.saturating_sub(1))
        } else {
            self.tab.saturating_add(1) % tabs_len
        };

        self.set_tab(new_tab)
    }

    fn switch_tab(&mut self, k: KeyEvent) -> Result<()> {
        if k == self.key_config.tab_status {
            self.set_tab(0)?;
        } else if k == self.key_config.tab_log {
            self.set_tab(1)?;
        } else if k == self.key_config.tab_files {
            self.set_tab(2)?;
        }
        // } else if k == self.key_config.tab_stashing {
        //     self.set_tab(3)?;
        // } else if k == self.key_config.tab_stashes {
        //     self.set_tab(4)?;
        // }

        Ok(())
    }

    fn set_tab(&mut self, tab: usize) -> Result<()> {
        let tabs = self.get_tabs();
        for (i, t) in tabs.into_iter().enumerate() {
            if tab == i {
                t.show()?;
            } else {
                t.hide();
            }
        }

        self.tab = tab;

        Ok(())
    }

    fn get_tabs(&mut self) -> Vec<&mut dyn Component> {
        vec![
            &mut self.t01,
            &mut self.t02,
            &mut self.t03,
            // &mut self.stashing_tab,
            // &mut self.stashlist_tab,
        ]
    }

    fn commands(&self, force_all: bool) -> Vec<CommandInfo> {
        let mut res = Vec::new();

        for c in self.components() {
            // c.commands(&mut res, true);
            if c.commands(&mut res, force_all)
                != CommandBlocking::PassingOn
                && !force_all
            {
                break;
            }
        }


        res.push(
            CommandInfo::new(
                strings::commands::toggle_tabs(&self.key_config),
                true,
                !self.any_popup_visible(),
            )
                .order(order::NAV),
        );
        res.push(
            CommandInfo::new(
                strings::commands::toggle_tabs_direct(
                    &self.key_config,
                ),
                true,
                !self.any_popup_visible(),
            )
                .order(order::NAV),
        );

        res.push(
            CommandInfo::new(
                strings::commands::quit(&self.key_config),
                true,
                !self.any_popup_visible(),
            )
                .order(127),
        );

        res
    }
}