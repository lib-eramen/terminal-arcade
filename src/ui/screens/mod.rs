//! [`Screen`]s - a core construct in Terminal Arcade for rendering and
//! receiving input. A screen (usually used through a [`ScreenHandle`] which is
//! where ) receives and produces events.

use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::{
		Event,
		ScreenEvent,
	},
	ui::screens::state::ScreenStateBuilder,
};

pub mod handle;
pub mod state;

pub use handle::ScreenHandle;
pub use state::ScreenState;

// FUTURE: When `typetag` supports associated types, switch to an `Either` API
// or the sorts with the events.

/// A screen that holds state, receives events and renders to the terminal.
#[typetag::serde(tag = "type")]
pub trait Screen: std::fmt::Debug {
	/// Returns the initial state that's associated with the screen.
	fn get_init_state<'a>(
		&self,
		builder: &'a mut ScreenStateBuilder,
	) -> &'a mut ScreenStateBuilder;

	/// Performs closing actions for the screen.
	/// The default behavior is just to send an event to finish the screen.
	fn close(
		&mut self,
		_state: &ScreenState,
		event_sender: &UnboundedSender<Event>,
	) -> crate::Result<()> {
		event_sender.send(ScreenEvent::Finish.into())?;
		Ok(())
	}

	/// Updates the screen's state.
	fn update(
		&mut self,
		_state: &ScreenState,
		_event_sender: &UnboundedSender<Event>,
	) -> crate::Result<()> {
		Ok(())
	}

	/// Handles an incoming [`Event`].
	fn event(
		&mut self,
		state: &ScreenState,
		event_sender: &UnboundedSender<Event>,
		event: Event,
	) -> crate::Result<()>;

	/// Renders this screen.
	fn render(
		&mut self,
		state: &mut ScreenState,
		frame: &mut Frame,
	) -> crate::Result<()>;
}
