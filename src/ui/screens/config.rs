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
			ui_presets::{
				titled_ui_block,
				untitled_ui_block,
			},
			under_construction::render_under_construction_block,
		},
		util::stylize,
		Screen,
	},
};

/// See the [module](self) documentation for more information.
#[derive(Default)]
pub struct ConfigScreen;

impl Screen for ConfigScreen {
	// TODO: I don't think this is getting implemented anytime soon...
	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		render_under_construction_block(frame);
	}
}
