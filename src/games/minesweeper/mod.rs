//! Implementation for the game Minesweeper.

use crossterm::event::Event;
use serde_derive::{
	Deserialize,
	Serialize,
};

use crate::{
	games::{
		Game,
		GameMetadata,
		GameState,
		GameStaticInfo,
		Games,
	},
	ui::{
		games::minesweeper::board_setup::MinesweeperSetupScreen,
		screens::Screens,
		Screen,
	},
};

/// The game [Minesweeper](https://en.wikipedia.org/wiki/Minesweeper_(video_game)).
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Minesweeper;

impl Game for Minesweeper {
	fn data(&self) -> GameState {
		GameState::new(
			GameMetadata::new(GameStaticInfo::new(
				self.clone().into(),
				"Minesweeper".to_string(),
				"A tile-based game of looking for mines and avoiding responsibilities.".to_string(),
				"0.0.1".to_string(),
			))
			.unwrap(),
			Some(MinesweeperSetupScreen::new().into()),
		)
	}

	fn event(&mut self, _event: &Event) -> anyhow::Result<()> {
		Ok(())
	}
}
