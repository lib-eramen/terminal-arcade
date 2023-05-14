//! The Minesweeper setup screen.

use ratatui::Frame;

use crate::{
	core::terminal::BackendType,
	ui::{
		components::{
			games::setup::SetupQuestion,
			scroll_tracker::ScrollTracker,
			ui_presets::titled_ui_block,
		},
		Screen,
	},
};

/// The struct containing the implmentation for the Minesweeper setup screen.
pub struct MinesweeperSetupScreen {
	/// The setup questions for this game.
	questions: Vec<SetupQuestion>,

	/// The scroll tracker for this screen.
	scroll_tracker: ScrollTracker,
}

impl Default for MinesweeperSetupScreen {
	fn default() -> Self {
		Self {
			questions: vec![],
			scroll_tracker: ScrollTracker::new(1, None),
		}
	}
}

impl Screen for MinesweeperSetupScreen {
	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		let size = frame.size();
		frame.render_widget(titled_ui_block("Minesweeper: Game setup"), size);
	}
}
