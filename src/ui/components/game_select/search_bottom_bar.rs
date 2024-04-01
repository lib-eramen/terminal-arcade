//! A bottom bar on the game selection screen that displays general info about
//! the search results.

use pluralizer::pluralize;
use ratatui::{
	layout::{
		Alignment,
		Rect,
	},
	text::Text,
	widgets::Paragraph,
	Frame,
};

use crate::{
	core::terminal::BackendType,
	ui::components::presets::untitled_ui_block,
};

/// Renders the bottom bar of the game selection screen.
pub fn render_search_bottom_bar(
	frame: &mut Frame<'_>,
	size: Rect,
	results_count: usize,
	time_to_search: f64,
	results_per_page: u64,
) {
	let bottom_bar_text = format!(
		"{}, in {}, displaying {} at once.",
		pluralize("result", results_count as isize, true),
		format!("{time_to_search} seconds"),
		format!("{results_per_page} results"),
	);
	let bottom_bar_paragraph =
		Paragraph::new(bottom_bar_text).block(untitled_ui_block()).alignment(Alignment::Center);
	frame.render_widget(bottom_bar_paragraph, size);
}
