//! A module containing the [Screen] trait, a trait needed to, basically, do
//! everything on the terminal in Terminal Arcade. See the [Screen] trait to get
//! started. It also contains the various screens that the game uses to present
//! itself on the terminal.

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
use ratatui::{
	buffer::Buffer,
	layout::{
		Constraint,
		Rect,
	},
	text::Text,
	widgets::{
		Cell,
		Clear,
		Row,
		Table,
		Widget,
	},
	Frame,
};

use crate::{
	core::terminal::BackendType,
	ui::{
		components::presets::highlight_block,
		util::stylize,
	},
};

pub mod config;
pub mod game_select;
pub mod games;
pub mod welcome;

/// A controls entry. The first element of the tuple is the key shortcut, while
/// the second element is the function (what it does in the context of the
/// screen).
pub type ControlsEntry = (String, String);

/// Checks if the event is a key event in which the ESC key is pressed.
#[must_use]
pub fn check_escape_key(event: &Event) -> bool {
	if let Event::Key(key) = event {
		key.code == KeyCode::Esc && key.modifiers == KeyModifiers::NONE
	} else {
		false
	}
}

/// Returns a table containing information about key shortcuts.
#[must_use]
pub fn get_controls_table<'a>(extra_entries: Option<Vec<ControlsEntry>>) -> Table<'a> {
	let mut entries = extra_entries.unwrap_or_default();
	let mut default_shortcuts = vec![
		(
			"Esc".to_string(),
			"Closes this screen and returns to the previous one".to_string(),
		),
		("Ctrl-Q".to_string(), "Quits the application".to_string()),
	];
	entries.append(&mut default_shortcuts);
	Table::new(entries.into_iter().map(|entry| Row::new([stylize(entry.0), Text::from(entry.1)])))
		.block(highlight_block(titled_ui_block("Controls")))
		.widths(&[
			Constraint::Ratio(1, 5), // shortcut
			Constraint::Ratio(4, 5), // function
		])
		.column_spacing(3)
		.header(Row::new(["Shortcut", "Function"]))
}

/// The trait for handling drawing on the terminal and receiving events from the
/// user.
/// One should always start here when making a game/screen.
#[must_use]
pub trait Screen {
	/// Called when an event is passed along to the screen,
	/// possibly from [`crate::TerminalArcade`], but also from whatever screen
	/// spawned this screen.
	fn event(&mut self, _event: &Event) -> anyhow::Result<()> {
		Ok(())
	}

	/// Called when the screen is being closed.
	/// This can be called when the entire application is being quit (in the
	/// proper manner, of course, not through a crash or a panic).
	fn close(&mut self) -> anyhow::Result<()> {
		Ok(())
	}

	/// Indicates that the screen is ready to be closed.
	/// If the screen is ready to be closed, the implementation of this function
	/// should return true. Otherwise, it should return false.
	fn is_closing(&self) -> bool {
		false
	}

	/// Indicates the screen that this screen itself is trying to create.
	/// If the window wants to create another screen, this function should
	/// return [Some], with the window inside it. Otherwise, return [None].
	fn screen_created(&mut self) -> Option<Box<dyn Screen>> {
		None
	}

	/// Returns extra entries coded in the page.
	/// It is helpful for screens to specify this, such that the controls popup
	/// works best.
	fn extra_controls_entries(&self) -> Option<Vec<ControlsEntry>> {
		None
	}

	/// Paints some UI to the screen.
	/// This method is also called when a resize event is triggered.
	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>);

	/// Draws the controls popup to the screen.
	/// This method is intended to be called whenever a shortcut is
	fn draw_controls_popup(&self, frame: &mut Frame<'_, BackendType>, buffer: &mut Buffer) {
		let frame_area = frame.size();
		let area = Rect {
			x: frame_area.width / 5,
			y: frame_area.height / 3,
			width: frame_area.width / 5 * 3,
			height: frame_area.height / 3,
		};
		Clear.render(area, buffer);
		frame.render_widget(get_controls_table(self.extra_controls_entries()), area);
	}
}

pub use config::ConfigScreen;
pub use game_select::GameSelectionScreen;
pub use games::*;
pub use welcome::WelcomeScreen;

use super::components::presets::titled_ui_block;
