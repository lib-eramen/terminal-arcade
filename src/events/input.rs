//! An input from the user that's supposed to change app state in some way.

use color_eyre::eyre::eyre;
use crossterm::event::{
	KeyEvent,
	MouseEvent,
};

use crate::events::{
	tui::FocusChange,
	TuiEvent,
};

/// An input from the user.
#[derive(Debug, Clone)]
pub enum InputEvent {
	/// The terminal is resized to `(width, height)`.
	ResizeTerminal(u16, u16),

	/// The terminal changed focus.
	ChangeFocus(FocusChange),

	/// Some text was pasted.
	PasteText(String),

	/// A key event.
	Key(KeyEvent),

	/// A mouse event. Note that mouse motion events are not handled.
	Mouse(MouseEvent),
}

impl TryFrom<TuiEvent> for InputEvent {
	type Error = color_eyre::Report;

	fn try_from(value: TuiEvent) -> Result<Self, Self::Error> {
		Ok(match value {
			TuiEvent::Resize(w, h) => Self::ResizeTerminal(w, h),
			TuiEvent::Focus(change) => Self::ChangeFocus(change),
			TuiEvent::Paste(text) => Self::PasteText(text),
			TuiEvent::Key(key) => Self::Key(key),
			TuiEvent::Mouse(mouse) => Self::Mouse(mouse),

			TuiEvent::Hello => {
				return Err(eyre!(
					"greetings! unfortunately the screens aren't much of a \
					 conversational mood right now. (or ever)"
				))
			},
			TuiEvent::Tick | TuiEvent::Render => {
				return Err(eyre!(
					"what the hell man, tick and render aren't input events"
				))
			},
		})
	}
}
