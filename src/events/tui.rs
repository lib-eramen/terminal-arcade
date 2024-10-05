//! Terminal events, sent by a [`Tui`](crate::tui::Tui).

use crossterm::event::Event as CrosstermEvent;

use crate::events::InputEvent;

/// Terminal events sent by [`Tui`](crate::tui::Tui).
///
/// Note that the inpu.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TuiEvent {
	/// Checks if event transmission works.
	Hello,

	/// Updates the application state.
	Tick,

	/// Renders the application to the terminal.
	Render,

	/// Terminal input event.
	Input(InputEvent),
}

impl TuiEvent {
	/// Returns whether this TUI event should be logged (e.g. not
	/// [`Tick`](TuiEvent::Tick) or [`Render`](TuiEvent::Render) since they are
	/// repetitive and potentially wasteful space-wise in a log file).
	pub fn should_be_logged(&self) -> bool {
		!matches!(self, Self::Render | Self::Tick)
	}
}

/// A change in focus of the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusChange {
	Lost,
	Gained,
}

impl From<CrosstermEvent> for InputEvent {
	fn from(value: CrosstermEvent) -> Self {
		match value {
			CrosstermEvent::Key(key) => Self::Key(key),
			CrosstermEvent::Mouse(mouse) => Self::Mouse(mouse),
			CrosstermEvent::Paste(text) => Self::Paste(text),
			CrosstermEvent::Resize(w, h) => Self::ResizeTerminal(w, h),
			CrosstermEvent::FocusLost => Self::ChangeFocus(FocusChange::Lost),
			CrosstermEvent::FocusGained => {
				Self::ChangeFocus(FocusChange::Gained)
			},
		}
	}
}

impl From<CrosstermEvent> for TuiEvent {
	fn from(value: CrosstermEvent) -> Self {
		Self::Input(value.into())
	}
}
