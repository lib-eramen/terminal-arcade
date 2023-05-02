//! A game-selection screen.
//! Users can scroll through the list with arrows to look for a game they want,
//! search a game by its name, or pick a game at random.

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
use ratatui::Frame;

use super::Screen;
use crate::{
	core::{
		terminal::BackendType,
		Outcome,
	},
	ui::components::presets::titled_ui_block,
};

/// The struct for the game selection screen.
#[derive(Default)]
pub struct GameSelectionScreen {
	/// Controls whether this screen is ready to be closed.
	closing: bool,
}

impl Screen for GameSelectionScreen {
	fn event(&mut self, event: &Event) -> Outcome<()> {
		if let Event::Key(key) = event {
			if key.code == KeyCode::Esc && key.modifiers == KeyModifiers::NONE {
				self.closing = true;
			}
		}
		Ok(())
	}

	fn is_closing(&self) -> bool {
		self.closing
	}

	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		let size = frame.size();
		frame.render_widget(titled_ui_block("Select a game!"), size);
	}
}

impl GameSelectionScreen {}
