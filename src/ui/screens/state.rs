//! Metadata for a [screen](Screens).

use serde::{
	Deserialize,
	Serialize,
};

use crate::ui::UiRunState;

/// A set of properties that always goes with every instance of a [`Screen`].
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenState {
	/// Run state of the screen.
	#[serde(skip)]
	pub run_state: UiRunState,

	/// Title of the screen.
	pub title: String,

	/// Whether the screen needs mouse input.
	pub needs_mouse: bool,
}

impl ScreenState {
	/// Constructs a new screen state object, with the run state set to
	/// [`SetState::Running`].
	pub fn new(title: String, needs_mouse: bool) -> Self {
		Self {
			run_state: UiRunState::Running,
			title,
			needs_mouse,
		}
	}
}
