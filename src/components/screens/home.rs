//! Home screen.

use ratatui::{
	widgets::Paragraph,
	Frame,
};
use serde::{
	Deserialize,
	Serialize,
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::instrument;

use crate::{
	events::{
		tui::InputEvent,
		AppEvent,
		Event,
	},
	ui::{
		screens::{
			Screen,
			ScreenState,
		},
		UiRunState,
	},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeScreen;

#[typetag::serde]
impl Screen for HomeScreen {
	/// Returns the initial state that's associated with the screen.
	fn get_init_state(&self) -> ScreenState {
		ScreenState::new("home sweet home!".to_string(), false)
	}

	/// Handles an incoming [`Event`].
	#[instrument(name = "home-screen")]
	fn event(
		&mut self,
		state: &mut ScreenState,
		event_sender: &UnboundedSender<Event>,
		event: &Event,
	) -> crate::Result<()> {
		if let Event::App(AppEvent::UserInputs(inputs)) = event {
			for input in inputs {
				if let InputEvent::Key(_key) = input {
					state.run_state = UiRunState::Finished;
					event_sender.send(Event::App(AppEvent::CloseApp))?;
				}
			}
		}
		Ok(())
	}

	/// Renders this screen.
	fn render(
		&mut self,
		_state: &mut ScreenState,
		frame: &mut Frame,
	) -> crate::Result<()> {
		frame.render_widget(Paragraph::new("helo, world!"), frame.size());
		Ok(())
	}
}
