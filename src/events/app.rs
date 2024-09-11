//! The most basic events that an [`App`](crate::app::App) will send and handle.

use color_eyre::{
	eyre::eyre,
	Section,
};

use crate::events::{
	tui::{
		FocusChange,
		InputEvent,
	},
	TuiEvent,
};

/// Events sent by [`Tui`].
#[derive(Debug, Clone)]
pub enum AppEvent {
	/// Updates the application state.
	Tick,

	/// Renders the application to the terminal.
	Render,

	/// Closes the application (not forcibly).
	CloseApp,

	/// Quits the application (forcibly).
	QuitApp,

	/// Closes the current active screen.
	CloseActiveScreen,

	/// An error occurred in the application, sent with the provided message.
	ErrorOccurred(String),

	/// The terminal changed focus.
	ChangeFocus(FocusChange),

	/// Some text was pasted.
	PasteText(String),

	/// The terminal is resized to `(width, height)`.
	ResizeTerminal(u16, u16),

	/// Inputs from the user, usually called as a tick finishes and the input
	/// event buffer is [drained](Vec::drain) and sent through here.
	UserInputs(Vec<InputEvent>),
}

impl AppEvent {
	/// Returns whether this event should be logged. This function will return
	/// `false` for repetitive app events ([`Self::Tick`] and [`Self::Render`])
	/// and for individual events that should be buffered and released with
	/// every app tick.
	pub fn should_be_logged(&self) -> bool {
		!matches!(self, Self::Render | Self::UserInputs(_))
	}
}

impl TryFrom<TuiEvent> for AppEvent {
	type Error = color_eyre::Report;

	/// Converts a [`TuiEvent`] to an [`AppEvent`]. Panics if the [`TuiEvent`]
	/// is a [`TuiEvent::Init`] event.
	fn try_from(
		value: TuiEvent,
	) -> Result<Self, <Self as TryFrom<TuiEvent>>::Error> {
		Ok(match value {
			TuiEvent::Tick => Self::Tick,
			TuiEvent::Render => Self::Render,
			TuiEvent::Focus(change) => Self::ChangeFocus(change),
			TuiEvent::Paste(text) => Self::PasteText(text),
			TuiEvent::Resize(w, h) => Self::ResizeTerminal(w, h),

			TuiEvent::Hello => {
				return Err(eyre!(
					"the tui said hi! unfortunately this is a language \
					 incomprehensible to `AppEvent` speakers."
				))
				.note("goodbye... i guess")
			},
			input @ TuiEvent::Input(_) => {
				return Err(eyre!(
					"cannot convert individual input event to app event"
				))
				.suggestion(
					"(dev) consider handling his variant directly or \
					 refactor. the code should already be utilizing \
					 `crate::event::TuiAppMiddleman` to handle these kinds of \
					 events.",
				)
				.with_note(|| format!("input event sent: {input:?}"))
			},
		})
	}
}
