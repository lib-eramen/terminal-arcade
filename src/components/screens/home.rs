//! Home screen.

use ratatui::Frame;
use serde::{
	Deserialize,
	Serialize,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::{
		Event,
		InputEvent,
		ScreenEvent,
	},
	ui::screens::{
		state::ScreenStateBuilder,
		Screen,
		ScreenState,
	},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeScreen;

#[typetag::serde]
impl Screen for HomeScreen {
	fn get_init_state<'a>(
		&self,
		builder: &'a mut ScreenStateBuilder,
	) -> &'a mut ScreenStateBuilder {
		builder.title("welcome home!")
	}

	fn event(
		&mut self,
		_state: &ScreenState,
		event_sender: &UnboundedSender<Event>,
		event: Event,
	) -> crate::Result<()> {
		if let Event::Input(InputEvent::Key(_)) = event {
			event_sender.send(ScreenEvent::Close.into())?;
		}
		Ok(())
	}

	fn render(
		&mut self,
		_state: &mut ScreenState,
		_frame: &mut Frame,
	) -> crate::Result<()> {
		Ok(())
	}
}
