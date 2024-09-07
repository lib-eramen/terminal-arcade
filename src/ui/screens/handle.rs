//! Wrapper struct for a [screen](Screens) and its [state](ScreenState).

use ratatui::Frame;
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::Event,
	ui::screens::{
		Screen,
		ScreenState,
	},
};

/// Wrapper struct for a [screen](Screens) and its [state](ScreenState).
#[derive(Debug, Serialize)]
pub struct ScreenHandle {
	/// Inner screen trait object.
	pub screen: Box<dyn Screen>,

	/// State associated with the screen.
	pub state: ScreenState,

	/// Event sender to the [`App`] layer.
	#[serde(skip)]
	event_sender: UnboundedSender<Event>,
}

impl ScreenHandle {
	/// Constructs a new handle from a screen and initializes state from
	/// [`Screen::get_init_state`].
	pub fn new<S>(screen: S, event_sender: UnboundedSender<Event>) -> Self
	where
		S: Screen + 'static,
	{
		let state = screen.get_init_state();
		Self {
			screen: Box::new(screen),
			state,
			event_sender,
		}
	}

	/// Handles an incoming event.
	pub fn event(&mut self, event: &Event) -> crate::Result<()> {
		self.screen
			.event(&mut self.state, &self.event_sender, event)
	}

	/// Renders the screen to the terminal.
	pub fn render(&mut self, frame: &mut Frame) -> crate::Result<()> {
		self.screen.render(&mut self.state, frame)
	}
}
