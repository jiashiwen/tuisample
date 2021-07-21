use std::{io, io::stdout, thread};
use std::error::Error;
use std::io::Write;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode}, ExecutableCommand, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::event::{KeyEvent, KeyModifiers};
use log::{debug, info};
use scopeguard::defer;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::Terminal;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};

use crate::app::App;
use crate::components::Component;
use crate::keys::KeyConfig;
use crate::logger::init_log;
use crate::ui::style::Theme;

#[allow(dead_code)]
mod app;
mod ui;
mod keys;
mod args;
mod bug_report;
mod cmdbar;
mod components;
mod strings;
mod tabs;
mod logger;


enum Event<I> {
    Input(I),
    Tick,
}


fn main() -> Result<(), Box<dyn Error>> {
    init_log();

    enable_raw_mode()?;
    let key_config = KeyConfig::init(KeyConfig::get_config_file()?)
        .map_err(|e| eprintln!("KeyConfig loading error: {}", e))
        .unwrap_or_default();
    // let theme = Theme::init(cliargs.theme)
    //     .map_err(|e| eprintln!("Theme loading error: {}", e))
    //     .unwrap_or_default();
    let theme = Theme::default();
    let mut stdout = stdout();


    setup_terminal()?;
    defer! {
        shutdown_terminal();
    }
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let mut app = App::new(false, theme, key_config);

    terminal.clear()?;

    loop {
        // terminal.draw(|f| myui::draw(f, &mut app))?;
        let emptymodifier = KeyModifiers::empty();
        draw(&mut terminal, &app)?;
        // app.cmdbar.borrow_mut().refresh_width(fsize.width);
        match rx.recv()? {
            Event::Input(event) => {
                app.event(event)?;
            }
            // Event::Input(event) => match event {
            //     KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL } => {
            //         disable_raw_mode()?;
            //         execute!(
            //             terminal.backend_mut(),
            //             LeaveAlternateScreen,
            //             DisableMouseCapture
            //         )?;
            //         terminal.show_cursor()?;
            //
            //         break;
            //     }
            //     KeyEvent { code: KeyCode::Char('1'), modifiers: emptymodifier } => {
            //         app.tab = 0;
            //
            //         app.update_commands();
            //     }
            //     KeyEvent { code: KeyCode::Char('2'), modifiers: emptymodifier } => {
            //         app.tab = 1;
            //         app.update_commands();
            //     }
            //     KeyEvent { code: KeyCode::Char('3'), modifiers: emptymodifier } => {
            //         app.tab = 2;
            //         app.update_commands();
            //     }
            //     _ => {
            //         match event.code {
            //             KeyCode::Tab => {
            //                 app.on_right();
            //                 app.update_commands();
            //             }
            //
            //             // KeyCode::Left => app.on_left(),
            //             // KeyCode::Right => app.on_right(),
            //
            //             KeyCode::Down => { app.update()? }
            //             _ => {}
            //         }
            //     }
            // }
            Event::Tick => {
                app.on_tick();
            }
        }

        if app.is_quit() {
            break;
        }
    }

    Ok(())
}

fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &App,
) -> io::Result<()> {
    if app.requires_redraw() {
        terminal.resize(terminal.size()?)?;
    }

    terminal.draw(|mut f| {
        if let Err(e) = app.draw(&mut f) {
            log::error!("failed to draw: {:?}", e);
        }
    })?;

    Ok(())
}

fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn shutdown_terminal() {
    let leave_screen =
        io::stdout().execute(LeaveAlternateScreen).map(|_f| ());

    if let Err(e) = leave_screen {
        eprintln!("leave_screen failed:\n{}", e);
    }

    let leave_raw_mode = disable_raw_mode();

    if let Err(e) = leave_raw_mode {
        eprintln!("leave_raw_mode failed:\n{}", e);
    }
}

fn start_terminal<W: Write>(
    buf: W,
) -> io::Result<Terminal<CrosstermBackend<W>>> {
    let backend = CrosstermBackend::new(buf);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}
