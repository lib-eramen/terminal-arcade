//! Functions for generating a list of controls shown at the welcome page.
//! In API signatures exposed here, WCL stands for "welcome controls list".

use crossterm::style::Attribute;
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
		Rect,
	},
	style::{
		Color,
		Style,
		Stylize,
	},
	text::Text,
	widgets::{
		BorderType,
		Borders,
		Paragraph,
	},
	Frame,
};

use crate::{
	core::terminal::BackendType,
	ui::components::presets::{
		highlight_block,
		titled_ui_block,
		untitled_ui_block,
	},
};

#[must_use]
fn controls_layout() -> Layout {
	Layout::default()
		.direction(Direction::Vertical)
		.vertical_margin(1)
		.horizontal_margin(0)
		.constraints(
			[
				Constraint::Max(3),
				Constraint::Max(3),
				Constraint::Max(3),
				Constraint::Max(0),
			]
			.as_ref(),
		)
}

#[must_use]
fn controls_paragraphs(selected: Option<u64>) -> Vec<Paragraph<'static>> {
	[
		"Hop into a game and play!",
		"View your configurations...",
		"Quit the game",
	]
	.into_iter()
	.enumerate()
	.map(|(index, text)| {
		let matches = selected.map_or(false, |selected_index| index as u64 == selected_index);
		Paragraph::new(text)
			.block(if matches { highlight_block(untitled_ui_block()) } else { untitled_ui_block() })
			.alignment(Alignment::Center)
	})
	.collect()
}

/// Renders the welcome list block.
pub fn render_welcome_controls_block(size: Rect, frame: &mut Frame<'_>, selected: Option<u64>) {
	frame.render_widget(titled_ui_block("Options").borders(Borders::NONE), size);
	let chunks = controls_layout().split(size);
	let widget_config = controls_paragraphs(selected).into_iter().zip(chunks.iter());
	for (paragraph, chunk) in widget_config {
		frame.render_widget(paragraph, *chunk);
	}
}
