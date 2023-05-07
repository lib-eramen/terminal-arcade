//! Implementation for the game Minesweeper.

use crossterm::event::Event;

use super::{
	Game,
	GameMetadata,
};
use crate::{
	core::Outcome,
	ui::{Screen, games::minesweeper::minesweeper_setup_screen::MinesweeperSetupScreen},
};

/// The struct containing the implementation for the game Minesweeper.
#[derive(Clone)]
pub struct Minesweeper;

impl Game for Minesweeper {
	fn metadata(&self) -> GameMetadata {
		GameMetadata::new(|info| {
			info.authors(vec!["me (the crate author), it literally says \"mine\" \
			                   in there"
				.to_string()])
				.description(
					"A tile-based game of looking for mines and avoiding responsibilities. On \
					 that note, get back to work, devs."
						.to_string(),
				)
				.name("Minesweeper".to_string())
				.version_created("0.0.1".to_string())
		})
		.unwrap()
	}

	fn is_finished(&self) -> bool {
		todo!()
	}

	fn event(&mut self, _event: &Event) -> Outcome<()> {
		todo!()
	}

	fn screen_created(&self) -> Box<dyn Screen> {
		Box::new(MinesweeperSetupScreen)
	}
}
