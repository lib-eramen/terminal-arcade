//! Events that manipulate the screen's
//! [state](crate::ui::screens::ScreenState).

use crate::ui::screens::ScreenHandle;

/// Screen [state](crate::ui::screens::ScreenState)-manipulating events.
#[derive(Debug)]
pub enum ScreenEvent {
	/// Sets the screen to [closing](crate::ui::UiRunState::Closing).
	Close,

	/// Marks the screen as finished and ready to be dropped.
	Finish,

	/// An error occurred in the application, sent with the provided message.
	Error(String),

	/// Updates the title of the screen.
	Rename(String),

	/// Create a new screen.
	Create(ScreenHandle),
}
