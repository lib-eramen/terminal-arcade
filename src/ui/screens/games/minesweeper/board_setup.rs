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

impl MinesweeperSetupScreen {
	/// Returns the layout for the Minesweeper board setup screen.
	#[must_use]
	fn board_setup_layout() -> Layout {
		let info_panel_height = 3 + 4;
		let constraints = vec![Constraint::Max(info_panel_height), Constraint::Min(0)];
		Layout::default()
			.direction(Direction::Vertical)
			.vertical_margin(1)
			.horizontal_margin(1)
			.constraints(constraints)
	}
}

impl Screen for MinesweeperSetupScreen {
	fn initial_state(&self) -> ScreenState {
		ScreenState::new("Mine your field!", ScreenKind::Normal, None)
	}

	fn render_screen(&mut self, frame: &mut Frame<'_>, _state: &ScreenState) {
		let size = frame.size();
		let chunks = Self::board_setup_layout().split(size);

		frame.render_widget(titled_ui_block("Controls"), chunks[0]);
	}

	fn event(&mut self, event: &Event, state: &mut ScreenState) -> anyhow::Result<()> {
		if let Event::Key(key) = event {
			if let KeyCode::Enter = key.code {
				state.open_status = OpenStatus::Closed;
			}
		}
		Ok(())
	}
}
