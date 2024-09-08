//! Terminal events, sent by a [`Tui`](crate::tui::Tui).

use color_eyre::{
	eyre::eyre,
	Section,
};
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

	/// Updates game state.
	Tick,

	/// Renders the application to the terminal.
	Render,

	/// The terminal is resized to `(width, height)`.
	Resize(u16, u16),

	/// The terminal changed focus.
	Focus(FocusChange),

	/// Some text was pasted.
	Paste(String),

	/// An input from the user.
	Input(InputEvent),
}

impl TuiEvent {
	/// Returns whether this TUI event should be logged (e.g. not
	/// [`Tick`](TuiEvent::Tick) or [`Render`](TuiEvent::Render) since they are
	/// repetitive and potentially wasteful space-wise in a log file).
	pub fn should_be_logged(&self) -> bool {
		!matches!(self, Self::Tick | Self::Render)
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
			input @ (CrosstermEvent::Key(_) | CrosstermEvent::Mouse(_)) => {
				Self::Input(input.try_into().unwrap())
			},
			CrosstermEvent::Paste(text) => Self::Paste(text),
			CrosstermEvent::Resize(w, h) => Self::Resize(w, h),
			CrosstermEvent::FocusLost => Self::Focus(FocusChange::Lost),
			CrosstermEvent::FocusGained => Self::Focus(FocusChange::Gained),
		}
	}
}

/// Input events that changes the app state and is buffered by the [`App`]
/// struct.
#[derive(Debug, Clone, Hash)]
pub enum InputEvent {
	/// A key is inputted by the user.
	Key(KeyEvent),

	/// The mouse is manipulated by the user.
	Mouse(MouseEvent),
}

impl TryFrom<CrosstermEvent> for InputEvent {
	type Error = color_eyre::Report;

	fn try_from(value: CrosstermEvent) -> Result<Self, Self::Error> {
		match value {
			CrosstermEvent::Key(key) => Ok(Self::Key(key)),
			CrosstermEvent::Mouse(mouse) => Ok(Self::Mouse(mouse)),
			event => Err(eyre!("cannot convert non-input tui event")
				.with_note(|| format!("trying to convert event: {event:?}"))),
		}
	}
}
