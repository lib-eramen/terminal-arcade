//! Game setup screen for a Minesweeper board.

use crossterm::event::{
	Event,
	KeyCode,
};
use derive_new::new;
use ratatui::{
	layout::{
		Constraint,
		Direction,
		Layout,
		Rect,
	},
	Frame,
};

use crate::ui::{
	components::presets::titled_ui_block,
	screen::{
		OpenStatus,
		ScreenKind,
		ScreenState,
	},
	Screen,
};

/// A setup screen for a board of Minesweeper.
#[derive(new, Clone)]
pub struct MinesweeperSetupScreen;

impl Screen for MinesweeperSetupScreen {
	fn initial_state(&self) -> ScreenState {
		ScreenState::new("Mine your field!", ScreenKind::Normal, None)
	}

	fn render_screen(&mut self, _frame: &mut Frame<'_>, _state: &ScreenState) {}

	fn event(&mut self, _event: &Event, _state: &mut ScreenState) -> anyhow::Result<()> {
		Ok(())
	}
}
