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

use crate::{
	core::terminal::BackendType,
	ui::{
		components::presets::titled_ui_block,
		Screen,
	},
};

/// A setup screen for a board of Minesweeper.
#[derive(new)]
pub struct MinesweeperSetupScreen {
	#[new(default)]
	setup_complete: bool,
}

impl MinesweeperSetupScreen {
	/// Returns the layout for the minesweeper board setup screen.
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
	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		let size = frame.size();
		frame.render_widget(titled_ui_block("Mine your field!"), size);
		let chunks = Self::board_setup_layout().split(size);

		frame.render_widget(titled_ui_block("Controls"), chunks[0]);
	}

	fn event(&mut self, event: &Event) -> anyhow::Result<()> {
		if let Event::Key(key) = event {
			match key.code {
				KeyCode::Enter => self.setup_complete = true,
				_ => {},
			}
		}
		Ok(())
	}

	fn is_closing(&self) -> bool {
		self.setup_complete
	}

	fn screen_created(&mut self) -> Option<Box<dyn Screen>> {
		None
	}
}
