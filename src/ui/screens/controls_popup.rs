//! Module for the controls popup.

use crossterm::event::Event;
use derive_new::new;
use ratatui::{
	layout::{Constraint, Rect},
	style::Modifier,
	widgets::{Cell, Clear, HighlightSpacing, Row, Table, Widget},
	Frame,
};

use crate::ui::{
	components::presets::{highlight_block, titled_ui_block, HIGHLIGHTED},
	screens::{ControlsEntry, ScreenKind, ScreenState},
	Screen,
};

/// A controls popup, consisting of only a [Table] listing out each controls
/// available at the page.
#[derive(Clone, new)]
pub struct ControlsPopup {
	extra_controls_entries: Option<Vec<ControlsEntry>>,
}

impl Screen for ControlsPopup {
	fn initial_state(&self) -> ScreenState {
		ScreenState::new(
			"Controls",
			ScreenKind::Popup,
			self.extra_controls_entries.clone(),
		)
	}

	fn handle_event(&mut self, _event: &Event, _state: &mut ScreenState) -> anyhow::Result<()> {
		Ok(())
	}

	fn render_ui(&self, frame: &mut Frame<'_>, state: &ScreenState) {
		let frame_area = frame.size();
		let buffer = frame.buffer_mut();
		let area = Rect {
			x: frame_area.width / 5,
			y: frame_area.height / 3,
			width: frame_area.width / 5 * 3,
			height: frame_area.height / 3,
		};
		Clear.render(area, buffer);
		frame.render_widget(
			Self::get_controls_table(state.controls_entries.clone()),
			area,
		);
	}
}

impl ControlsPopup {
	/// Returns a table containing information about key shortcuts.
	#[must_use]
	fn get_controls_table<'a>(extra_entries: Option<Vec<ControlsEntry>>) -> Table<'a> {
		// TODO: Replace this with own controls table widget
		let mut entries = extra_entries.unwrap_or_default();
		let mut default_shortcuts = vec![
			("Esc", "Closes this screen and returns to the previous one"),
			("Ctrl-Q", "Quits the application"),
		];
		entries.append(&mut default_shortcuts);
		Table::new(
			entries.into_iter().map(|entry| Row::new([Cell::new(entry.0), Cell::new(entry.1)])),
			&[
				Constraint::Ratio(1, 6), // shortcut
				Constraint::Ratio(5, 6), // function
			],
		)
		.block(highlight_block(titled_ui_block("Controls")))
		.highlight_spacing(HighlightSpacing::Always)
		.column_spacing(3)
		.header(
			Row::new(["Shortcut", "Function"])
				.style(HIGHLIGHTED.add_modifier(Modifier::UNDERLINED)),
		)
	}
}
