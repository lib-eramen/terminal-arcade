//! The screen for viewing and modifying the configuration for Terminal Arcade.

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
	},
	widgets::{
		Borders,
		Paragraph,
	},
	Frame,
};

use crate::{
	core::terminal::BackendType,
	ui::{
		components::{
			presets::{
				titled_ui_block,
				untitled_ui_block,
			},
			under_construction::render_under_construction_block,
		},
		screen::{
			ScreenKind,
			ScreenState,
		},
		Screen,
	},
};

/// See the [module](self) documentation for more information.
#[derive(Default, Clone)]
pub struct ConfigScreen;

impl Screen for ConfigScreen {
	fn initial_state(&self) -> ScreenState {
		ScreenState::new("Under construction!", ScreenKind::Normal, None)
	}

	fn event_screen(&mut self, _event: &Event, _state: &mut ScreenState) -> anyhow::Result<()> {
		Ok(())
	}

	// TODO: why are there only 24 hours in a day
	fn render_screen(&mut self, frame: &mut Frame<'_>, _state: &ScreenState) {
		render_under_construction_block(frame);
	}
}
