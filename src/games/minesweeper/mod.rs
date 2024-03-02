//! Implementation for the game Minesweeper.

use crossterm::event::Event;

use crate::{
	games::{
		Game,
		GameMetadata,
	},
	ui::{
		games::minesweeper::board_setup::MinesweeperSetupScreen,
		Screen,
	},
};

/// The struct containing the implementation for the game Minesweeper.
#[derive(Clone)]
pub struct Minesweeper;

impl Game for Minesweeper {
	fn metadata(&self) -> GameMetadata {
		GameMetadata::new(|info| {
			info.description(
				"A tile-based game of looking for mines and avoiding responsibilities.".to_string(),
			)
			.name("Minesweeper".to_string())
			.version_created("0.0.1".to_string())
		})
		.unwrap()
	}

	fn is_finished(&self) -> bool {
		false
	}

	fn event(&mut self, _event: &Event) -> anyhow::Result<()> {
		Ok(())
	}

	fn screen_created(&self) -> Box<dyn Screen> {
		Box::new(MinesweeperSetupScreen::new())
	}
}
