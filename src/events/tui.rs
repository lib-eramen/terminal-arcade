//! Terminal events, sent by a [`Tui`](crate::tui::Tui).

use crossterm::event::{
	Event as CrosstermEvent,
	KeyEvent,
	MouseEvent,
};

/// Terminal events sent by [`Tui`](crate::tui::Tui).
#[derive(Debug, Clone, Hash)]
pub enum TuiEvent {
	/// Checks if event transmission works.
	Hello,

	/// Updates the application state.
	Tick,

	/// Renders the application to the terminal.
	Render,

	/// The terminal is resized to `(width, height)`.
	Resize(u16, u16),

	/// The terminal changed focus.
	Focus(FocusChange),

	/// Some text was pasted.
	Paste(String),

	/// A key is inputted by the user.
	Key(KeyEvent),

	/// The mouse is manipulated by the user.
	Mouse(MouseEvent),
}

impl TuiEvent {
	/// Returns whether this TUI event should be logged (e.g. not
	/// [`Tick`](TuiEvent::Tick) or [`Render`](TuiEvent::Render) since they are
	/// repetitive and potentially wasteful space-wise in a log file).
	pub fn should_be_logged(&self) -> bool {
		!matches!(self, Self::Render)
	}
}

/// A change in focus of the terminal.
#[derive(Debug, Clone, Copy, Hash)]
#[allow(missing_docs)] // Obvious variant names
pub enum FocusChange {
	Lost,
	Gained,
}

impl From<CrosstermEvent> for TuiEvent {
	fn from(value: CrosstermEvent) -> Self {
		match value {
			CrosstermEvent::Key(key) => Self::Key(key),
			CrosstermEvent::Mouse(mouse) => Self::Mouse(mouse),
			CrosstermEvent::Paste(text) => Self::Paste(text),
			CrosstermEvent::Resize(w, h) => Self::Resize(w, h),
			CrosstermEvent::FocusLost => Self::Focus(FocusChange::Lost),
			CrosstermEvent::FocusGained => Self::Focus(FocusChange::Gained),
		}
	}
}
