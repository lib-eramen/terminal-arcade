//! The most basic events that an [`App`](crate::app::App) will send and handle.

use crate::events::InputEvent;

/// Events sent by the [`App`](crate::app::App).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppEvent {
	/// Updates the application state. This variant contains the inputs
	/// accumulated in that one tick.
	Tick(Vec<InputEvent>),

	/// Renders the application to the terminal.
	Render,

	/// Closes the application (not forcibly).
	Close,

	/// Quits the application (forcibly).
	Quit,

	/// An error occurred in the application, sent with the provided message.
	Error(String),
}

impl AppEvent {
	/// Returns whether this event should be logged. This function will return
	/// `false` for repetitive app events ([`Self::Tick`] and [`Self::Render`])
	/// and for individual events that should be buffered and released with
	/// every app tick.
	pub fn should_be_logged(&self) -> bool {
		!matches!(self, Self::Render | Self::Tick(_))
	}
}
