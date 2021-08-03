use std::convert::From;

use anyhow::Result;
use crossterm::event::Event;
use tui::{
    backend::Backend,
    Frame,
    layout::{Alignment, Rect},
    text::{Span, Text},
    widgets::{Block, Borders, BorderType, Paragraph, Wrap},
};

pub use command::{CommandInfo, CommandText};
pub use help::HelpComponent;
pub use list::ListComponent;
pub use search::InputMode;
pub use search::SearchComponent;

use crate::ui::style::Theme;

mod command;
mod help;
mod search;
mod list;


/// creates accessors for a list of components
///
/// allows generating code to make sure
/// we always enumerate all components in both getter functions
#[macro_export]
macro_rules! accessors {
    ($self:ident, [$($element:ident),+]) => {
        fn components(& $self) -> Vec<&dyn Component> {
            vec![
                $(&$self.$element,)+
            ]
        }

        fn components_mut(&mut $self) -> Vec<&mut dyn Component> {
            vec![
                $(&mut $self.$element,)+
            ]
        }
    };
}

/// creates a function to determine if any popup is visible
#[macro_export]
macro_rules! any_popup_visible {
    ($self:ident, [$($element:ident),+]) => {
        fn any_popup_visible(& $self) -> bool{
            ($($self.$element.is_visible()) || +)
        }
    };
}

/// creates the draw popup function
#[macro_export]
macro_rules! draw_popups {
    ($self:ident, [$($element:ident),+]) => {
        fn draw_popups<B: Backend>(& $self, mut f: &mut Frame<B>) -> Result<()>{
            //TODO: move the layout part out and feed it into `draw_popups`
            let size = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length($self.cmdbar.borrow().height()),
                ]
                .as_ref(),
            )
            .split(f.size())[0];

            ($($self.$element.draw(&mut f, size)?) , +);

            return Ok(());
        }
    };
}

/// simply calls
/// any_popup_visible!() and draw_popups!() macros
#[macro_export]
macro_rules! setup_popups {
    ($self:ident, [$($element:ident),+]) => {
        crate::any_popup_visible!($self, [$($element),+]);
        crate::draw_popups!($self, [ $($element),+ ]);
    };
}

/// returns `true` if event was consumed
pub fn event_pump(
    ev: Event,
    components: &mut [&mut dyn Component],
) -> Result<EventState> {
    for c in components {
        if c.event(ev)?.is_consumed() {
            return Ok(EventState::Consumed);
        }
    }

    Ok(EventState::NotConsumed)
}

/// helper fn to simplify delegating command
/// gathering down into child components
/// see `event_pump`,`accessors`
pub fn command_pump(
    out: &mut Vec<CommandInfo>,
    force_all: bool,
    components: &[&dyn Component],
) {
    for c in components {
        if c.commands(out, force_all) != CommandBlocking::PassingOn
            && !force_all
        {
            break;
        }
    }
}

#[derive(Copy, Clone)]
pub enum ScrollType {
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
}

///
#[derive(PartialEq)]
pub enum CommandBlocking {
    Blocking,
    PassingOn,
}

///
pub fn visibility_blocking<T: Component>(
    comp: &T,
) -> CommandBlocking {
    if comp.is_visible() {
        CommandBlocking::Blocking
    } else {
        CommandBlocking::PassingOn
    }
}

///
pub trait DrawableComponent {
    ///
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()>;
}

///
#[derive(PartialEq)]
pub enum EventState {
    Consumed,
    NotConsumed,
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }
}

impl From<bool> for EventState {
    fn from(consumed: bool) -> Self {
        if consumed {
            Self::Consumed
        } else {
            Self::NotConsumed
        }
    }
}

/// base component trait
pub trait Component {
    ///
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking;

    ///
    fn event(&mut self, ev: Event) -> Result<EventState>;

    ///
    fn focused(&self) -> bool {
        false
    }
    /// focus/unfocus this component depending on param
    fn focus(&mut self, _focus: bool) {}
    ///
    fn is_visible(&self) -> bool {
        true
    }
    ///
    fn hide(&mut self) {}
    ///
    fn show(&mut self) -> Result<()> {
        Ok(())
    }

    ///
    fn toggle_visible(&mut self) -> Result<()> {
        if self.is_visible() {
            self.hide();
            Ok(())
        } else {
            self.show()
        }
    }
}

fn dialog_paragraph<'a>(
    title: &'a str,
    content: Text<'a>,
    theme: &Theme,
    focused: bool,
) -> Paragraph<'a> {
    Paragraph::new(content)
        .block(
            Block::default()
                .title(Span::styled(title, theme.title(focused)))
                .borders(Borders::ALL)
                .border_style(theme.block(focused)),
        )
        .alignment(Alignment::Left)
}

fn popup_paragraph<'a, T>(
    title: &'a str,
    content: T,
    theme: &Theme,
    focused: bool,
) -> Paragraph<'a>
    where
        T: Into<Text<'a>>,
{
    Paragraph::new(content.into())
        .block(
            Block::default()
                .title(Span::styled(title, theme.title(focused)))
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(theme.block(focused)),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
}
