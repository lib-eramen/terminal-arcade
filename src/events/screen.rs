//! Events that manipulate the screen's
//! [state](crate::ui::screens::ScreenState).

/// Screen [state](crate::ui::screens::ScreenState)-manipulating events.
#[derive(Debug, Clone)]
pub enum ScreenEvent {
	/// Sets the screen to [closing](crate::ui::UiRunState::Closing).
	Close,

	/// Marks the screen as finished and ready to be dropped.
	Finish,

	/// Updates the title of the screen.
	UpdateTitle(String),
}
