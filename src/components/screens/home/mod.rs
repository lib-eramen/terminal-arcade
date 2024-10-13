//! Home screen to greet a user on running.

use ratatui::{
	layout::Rect,
	Frame,
};

use crate::{
	events::{
		Event,
		InputEvent,
		ScreenEvent,
	},
	ui::{
		screens::{
			handle::ScreenHandleData,
			state::ScreenDataBuilder,
			Screen,
		},
		UiElement,
	},
};

#[derive(Debug)]
pub struct HomeScreen;

impl UiElement for HomeScreen {
	type State = ScreenHandleData;

	fn event(
		&mut self,
		handle: Self::State,
		event: Event,
	) -> crate::Result<()> {
		if let Event::Input(InputEvent::Key(_)) = event {
			handle.event_sender.send(ScreenEvent::Close.into())?;
		}
		Ok(())
	}

	fn render(&self, _state: Self::State, _frame: &mut Frame<'_>, _size: Rect) {
	}
}

impl Screen for HomeScreen {
	fn get_init_state<'a>(
		&self,
		builder: &'a mut ScreenDataBuilder,
	) -> &'a mut ScreenDataBuilder {
		builder.title("Terminal Arcade 🕹️")
	}
}
