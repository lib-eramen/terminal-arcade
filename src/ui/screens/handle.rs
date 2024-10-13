//! Wrapper struct for a [screen](Screens) and its [state](ScreenState).

use derive_new::new;
use ratatui::{
	layout::Rect,
	Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::{
		AppEvent,
		Event,
		ScreenEvent,
	},
	ui::{
		screens::{
			state::ScreenDataBuilderError,
			Screen,
			ScreenData,
		},
		UiRunState,
	},
};

/// Wrapper struct for a [screen](Screens) and its [state](ScreenState).
#[derive(Debug)]
pub struct ScreenHandle {
	/// Inner screen trait object.
	pub screen: Box<dyn Screen>,

	/// Data associated with the screen.
	pub data: ScreenData,

	/// Event sender to the [`App`] layer.
	pub event_sender: UnboundedSender<Event>,
}

impl ScreenHandle {
	/// Constructs a new handle from a screen and initializes state from
	/// [`Screen::get_init_state`].
	pub fn new<S>(
		screen: S,
		event_sender: UnboundedSender<Event>,
	) -> Result<Self, ScreenDataBuilderError>
	where
		S: Screen + 'static,
	{
		let mut state_builder = ScreenData::builder();
		let state = screen.get_init_state(&mut state_builder).build()?;
		Ok(Self {
			screen: Box::new(screen),
			data: state,
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
				self.data.run_state = UiRunState::Closing;
				self.screen.close(self.clone_handle_state())?;
			},
			ScreenEvent::Finish => self.data.run_state = UiRunState::Finished,
			ScreenEvent::Rename(title) => {
				self.data.title.clone_from(title);
			},
			ScreenEvent::Error(_error) => todo!(),
			ScreenEvent::Create(_screen_handle) => todo!(),
		}
		Ok(())
	}

	/// Updates the screen.
	pub fn update(&mut self) -> crate::Result<()> {
		self.screen.update(self.clone_handle_state())
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
			let state = self.clone_handle_state();
			Screen::event(self.screen.as_mut(), state, event)?;
		}
		Ok(())
	}

	/// Renders the screen to the terminal.
	pub fn render(&self, frame: &mut Frame<'_>, size: Rect) {
		let state = self.clone_handle_state();
		Screen::render(self.screen.as_ref(), state, frame, size);
	}

	pub fn clone_handle_state(&self) -> ScreenHandleData {
		ScreenHandleData::new(self.data.clone(), self.event_sender.clone())
	}
}

/// Cloned fields from [`ScreenHandle`].
///
/// The DRY pastors are fuming.
#[derive(new)]
pub struct ScreenHandleData {
	pub state: ScreenData,
	pub event_sender: UnboundedSender<Event>,
}
