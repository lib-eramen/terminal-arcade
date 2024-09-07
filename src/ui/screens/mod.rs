//! Screens - a core construct in Terminal Arcade for rendering and receiving
//! input. A screen (usually used through a [`ScreenHandle`] which is where
//! screens works best) receives and produces events.

//! Handler to manage screens and rendering them.

use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use crate::events::Event;

pub mod handle;
pub mod handler;
pub mod state;

pub use handle::ScreenHandle;
pub use handler::ScreenHandler;
pub use state::ScreenState;

// FUTURE: When `typetag` supports associated types, switch to an `Either` API
// or the sorts with the events.

/// A screen that holds state, receives events and renders to the terminal.
#[typetag::serde(tag = "type")]
pub trait Screen: std::fmt::Debug {
	/// Returns the initial state that's associated with the screen.
	fn get_init_state(&self) -> ScreenState;

	/// Handles an incoming [`Event`].
	fn event(
		&mut self,
		state: &mut ScreenState,
		event_sender: &UnboundedSender<Event>,
		event: &Event,
	) -> crate::Result<()>;

	/// Renders this screen.
	fn render(
		&mut self,
		state: &mut ScreenState,
		frame: &mut Frame,
	) -> crate::Result<()>;
}
