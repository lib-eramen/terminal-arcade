//! Wrapper struct for a [screen](Screens) and its [state](ScreenState).

use ratatui::Frame;
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::{
		AppEvent,
		Event,
		ScreenEvent,
	},
	ui::{
		screens::{
			state::{
				ScreenStateBuilder,
				ScreenStateBuilderError,
			},
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
	pub event_sender: UnboundedSender<Event>,
}

impl ScreenHandle {
	/// Constructs a new handle from a screen and initializes state from
	/// [`Screen::get_init_state`].
	pub fn new<S>(
		screen: S,
		event_sender: UnboundedSender<Event>,
	) -> Result<Self, ScreenStateBuilderError>
	where
		S: Screen + 'static,
	{
		let mut state_builder = ScreenStateBuilder::default();
		let state = screen.get_init_state(&mut state_builder).build()?;
		Ok(Self {
			screen: Box::new(screen),
			state,
			event_sender,
		})
	}

	/// Handles an incoming [`ScreenEvent`].
	fn handle_screen_event(
		&mut self,
		event: &ScreenEvent,
	) -> crate::Result<()> {
		match event {
			ScreenEvent::Close => {
				self.state.run_state = UiRunState::Closing;
				self.screen.close(&self.state, &self.event_sender)?;
			},
			ScreenEvent::Finish => self.state.run_state = UiRunState::Finished,
			ScreenEvent::Rename(title) => {
				self.state.title.clone_from(title);
			},
		}
		Ok(())
	}

	/// Updates the screen.
	pub fn update(&mut self) -> crate::Result<()> {
		self.screen.update(&self.state, &self.event_sender)
	}

	/// Handles an incoming event.
	pub fn event(&mut self, event: Event) -> crate::Result<()> {
		let events = match event {
			Event::Screen(screen_event) => {
				self.handle_screen_event(&screen_event)?;
				vec![screen_event.clone().into()]
			},
			Event::App(AppEvent::Tick(input_events)) => {
				input_events.into_iter().map(Event::Input).collect()
			},
			_ => vec![event],
		};
		for event in events {
			self.screen.event(&self.state, &self.event_sender, event)?;
		}
		Ok(())
	}

	/// Renders the screen to the terminal.
	pub fn render(&mut self, frame: &mut Frame) -> crate::Result<()> {
		self.screen.render(&mut self.state, frame)
	}
}
