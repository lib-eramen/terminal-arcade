//! The most basic events that an [`App`](crate::app::App) will send and handle.

use color_eyre::{
	eyre::eyre,
	Section,
};

use crate::events::{
	InputEvent,
	ScreenEvent,
	TuiEvent,
};

/// Events sent by the [`App`](crate::app::App).
#[derive(Debug, Clone)]
pub enum AppEvent {
	/// Updates the application state.
	Tick,

	/// Renders the application to the terminal.
	Render,

	/// Closes the application (not forcibly).
	Close,

	/// Quits the application (forcibly).
	Quit,

	/// An error occurred in the application, sent with the provided message.
	ErrorOccurred(String),

	/// Input from the user, to be forwarded to screens to handle.
	UserInput(InputEvent),

	/// The screen is manipulated.
	ManipulateScreen(ScreenEvent),
}

impl AppEvent {
	/// Returns whether this event should be logged. This function will return
	/// `false` for repetitive app events ([`Self::Tick`] and [`Self::Render`])
	/// and for individual events that should be buffered and released with
	/// every app tick.
	pub fn should_be_logged(&self) -> bool {
		!matches!(self, Self::Render)
	}
}

impl TryFrom<TuiEvent> for AppEvent {
	type Error = color_eyre::Report;

	/// Converts a [`TuiEvent`] to an [`AppEvent`]. Panics if the [`TuiEvent`]
	/// is a [`TuiEvent::Init`] event.
	#[allow(clippy::expect_used, reason = "infallible")]
	fn try_from(
		value: TuiEvent,
	) -> Result<Self, <Self as TryFrom<TuiEvent>>::Error> {
		Ok(match value {
			TuiEvent::Tick => Self::Tick,
			TuiEvent::Render => Self::Render,

			#[rustfmt::skip]
			input @ (
				TuiEvent::Resize(..)
				| TuiEvent::Focus(_)
				| TuiEvent::Paste(_)
				| TuiEvent::Key(_)
				| TuiEvent::Mouse(_)
			) => Self::UserInput(
				input
					.try_into()
					.expect("could not convert tui event into input event"),
			),

			TuiEvent::Hello => {
				return Err(eyre!(
					"the tui said hi! unfortunately this is a language \
					 incomprehensible to the `App`."
				))
				.note("goodbye... i guess")
			},
		})
	}
}
