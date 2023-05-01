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

use super::Screen;
use crate::{
	core::{
		terminal::BackendType,
		Outcome,
	},
	ui::{
		components::{
			presets::{
				titled_ui_block,
				untitled_ui_block,
			},
			uc::draw_uc_block,
		},
		util::stylize,
	},
};

/// See the [module](self) documentation for more information.
#[derive(Default)]
pub struct ConfigScreen {
	/// Controls whether the screen is closing or not.
	closing: bool,
}

impl Screen for ConfigScreen {
	fn event(&mut self, event: &Event) -> Outcome<()> {
		if let Event::Key(key) = event {
			if key.code == KeyCode::Esc && key.modifiers == KeyModifiers::NONE {
				self.closing = true;
			}
		}
		Ok(())
	}

	fn is_closing(&self) -> bool {
		self.closing
	}

	// TODO: I don't think this is getting implemented anytime soon...
	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		draw_uc_block(frame);
	}
}
