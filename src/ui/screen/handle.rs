//! Wrapper struct for a [screen](Screens) and its [state](ScreenState).

use std::sync::{
	Arc,
	Mutex,
};

use ratatui::{
	layout::Rect,
	Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::metadata::{
	ScreenMetadata,
	ScreenMetadataBuilderError,
};
use crate::{
	events::{
		AppEvent,
		Event,
		ScreenEvent,
	},
	ui::{
		screen::Screen,
		UiRunState,
	},
};

/// Wrapper struct for a [screen](Screens) and its [state](ScreenState).
#[derive(Debug)]
pub struct ScreenHandle {
	/// Inner screen trait object.
	pub screen: Box<dyn Screen>,

	/// Metadata associated with the screen.
	pub metadata: Arc<Mutex<ScreenMetadata>>,

	/// Event sender to the [`App`] layer.
	pub event_sender: UnboundedSender<Event>,
}

impl ScreenHandle {
	/// Constructs a new handle from a screen and initializes state from
	/// [`Screen::get_init_state`].
	pub fn new<S>(
		screen: S,
		event_sender: UnboundedSender<Event>,
	) -> Result<Self, ScreenMetadataBuilderError>
	where
		S: Screen + 'static,
	{
		let mut metadata_builder = ScreenMetadata::builder();
		let metadata = screen.get_init_state(&mut metadata_builder).build()?;
		Ok(Self {
			screen: Box::new(screen),
			metadata: Arc::new(Mutex::new(metadata)),
			event_sender,
		})
	}

	/// Handles an incoming [`ScreenEvent`].
	fn handle_screen_event(
		&mut self,
		event: &ScreenEvent,
	) -> crate::Result<()> {
		let mut metadata = self.metadata.lock().unwrap();
		match event {
			ScreenEvent::Close => {
				metadata.run_state = UiRunState::Closing;
				self.screen.close(&mut self.clone_handle_state())?;
			},
			ScreenEvent::Finish => metadata.run_state = UiRunState::Finished,
			ScreenEvent::Rename(title) => {
				metadata.title.clone_from(title);
			},
			ScreenEvent::Error(_) | ScreenEvent::Create(_) => {},
		}
		Ok(())
	}

	/// Updates the screen with one tick.
	pub fn tick(&mut self) -> crate::Result<()> {
		let mut state = self.clone_handle_state();
		Screen::tick(self.screen.as_mut(), &mut state)
	}

	/// Handles an incoming event.
	pub fn event(&mut self, event: Event) -> crate::Result<()> {
		let events = match event {
			Event::Screen(screen_event) => {
				self.handle_screen_event(&screen_event)?;
				vec![screen_event.into()]
			},
			Event::App(AppEvent::Tick(input_events)) => {
				input_events.into_iter().map(Event::Input).collect()
			},
			_ => vec![event],
		};
		for event in events {
			let mut state = self.clone_handle_state();
			Screen::event(self.screen.as_mut(), &mut state, &event)?;
		}
		Ok(())
	}

	/// Renders the screen to the terminal.
	pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
		let mut state = self.clone_handle_state();
		Screen::render(self.screen.as_ref(), &mut state, frame, area);
	}

	pub fn clone_handle_state(&self) -> ScreenHandleData {
		ScreenHandleData {
			metadata: self.metadata.clone(),
			event_sender: self.event_sender.clone(),
		}
	}
}

/// Cloned fields from [`ScreenHandle`].
///
/// The DRY pastors are fuming.
pub struct ScreenHandleData {
	pub metadata: Arc<Mutex<ScreenMetadata>>,
	pub event_sender: UnboundedSender<Event>,
}
