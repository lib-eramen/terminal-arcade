//! Wrapper struct for a [screen](Screens) and its [state](ScreenState).

use ratatui::Frame;
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::{
		Event,
		ScreenEvent,
	},
	ui::{
		screens::{
			Screen,
			ScreenState,
		},
		UiRunState,
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

	/// Handles an incoming [`ScreenEvent`].
	fn handle_screen_event(&mut self, event: &ScreenEvent) {
		match event {
			ScreenEvent::Close => self.state.run_state = UiRunState::Closing,
			ScreenEvent::Finish => self.state.run_state = UiRunState::Finished,
			ScreenEvent::UpdateTitle(title) => {
				self.state.title.clone_from(title);
			},
		}
	}

	/// Handles an incoming event.
	pub fn event(&mut self, event: &Event) -> crate::Result<()> {
		if let Event::Screen(screen_event) = event {
			self.handle_screen_event(screen_event);
		}
		self.screen
			.handle_event(&self.state, &self.event_sender, event)
	}

	/// Renders the screen to the terminal.
	pub fn render(&mut self, frame: &mut Frame) -> crate::Result<()> {
		self.screen.render(&mut self.state, frame)
	}
}
