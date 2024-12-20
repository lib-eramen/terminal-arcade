//! Terminal events, sent by a [`Station`](crate::station::Station).

use crossterm::event::Event as CrosstermEvent;

use crate::events::InputEvent;

/// Terminal events sent by [`Station`](crate::station::Station).
#[derive(Debug, Clone)]
pub enum StationEvent {
	/// Checks if event transmission works.
	Hello,

	/// Updates the application state.
	Tick,

	/// Renders the application to the terminal.
	Render,

	/// Terminal input event.
	Input(InputEvent),
}

impl StationEvent {
	/// Returns whether this station event should be logged (e.g. not
	/// [`Tick`](StationEvent::Tick) or [`Render`](StationEvent::Render) since
	/// they are repetitive and potentially wasteful space-wise in a log file).
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

impl From<CrosstermEvent> for StationEvent {
	fn from(value: CrosstermEvent) -> Self {
		Self::Input(value.into())
	}
}
