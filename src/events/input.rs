//! An input from the user that's supposed to change app state in some way.

use crossterm::event::{
	KeyEvent,
	MouseEvent,
};

use crate::events::tui::FocusChange;

/// An input from the user.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputEvent {
	/// The terminal is resized to `(width, height)`.
	ResizeTerminal(u16, u16),

	/// The terminal changed focus.
	ChangeFocus(FocusChange),

	/// Some text was pasted.
	Paste(String),

	/// A key event.
	Key(KeyEvent),

	/// A mouse event.
	Mouse(MouseEvent),
}
