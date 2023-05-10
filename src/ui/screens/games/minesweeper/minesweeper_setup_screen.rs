//! The Minesweeper setup screen.

use ratatui::Frame;

use crate::{
	core::terminal::BackendType,
	ui::{
		components::ui_presets::titled_ui_block,
		Screen,
	},
};

/// The struct containing the implmentation for the Minesweeper setup screen.
pub struct MinesweeperSetupScreen;

impl Screen for MinesweeperSetupScreen {
	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		let size = frame.size();
		frame.render_widget(titled_ui_block("Minesweeper: Game setup"), size);
	}
}
