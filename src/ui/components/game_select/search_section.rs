//! A search bar with a back "button" (in actuality it's just help text)
//! and another row with the help text for the random selection function.

use ansi_to_tui::IntoText;
use crossterm::style::Attribute;
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
		Rect,
	},
	text::Text,
	widgets::Paragraph,
	Frame,
};

use crate::{
	core::terminal::BackendType,
	games::Game,
	ui::{
		components::presets::{
			titled_ui_block,
			untitled_ui_block,
		},
		util::{
			stylize,
			stylize_raw,
		},
	},
};

#[must_use]
fn search_section_layout() -> Layout {
	Layout::default()
		.direction(Direction::Vertical)
		.vertical_margin(0)
		.horizontal_margin(0)
		.constraints(vec![
			Constraint::Max(3), // Back "button" and search bar
			Constraint::Max(0), // Prevent blocks from taking up remaining space
		])
}

/// Renders the top row of the search bar section.
pub fn render_search_bar_top_row(frame: &mut Frame<'_>, size: Rect, search_term: Option<&str>) {
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.margin(0)
		.constraints([
			Constraint::Max(13), // Back button (does nothing)
			Constraint::Min(1),  // Search area
		])
		.split(size);

	let back_button =
		Paragraph::new(stylize("⏪ Back")).alignment(Alignment::Center).block(untitled_ui_block());
	frame.render_widget(back_button, chunks[0]);

	let search_bar_text = search_term.map_or_else(
		|| stylize(" 🔎︎ Search..."),
		|term| stylize(format!(" 🔎︎ {term}█")),
	);
	let search_bar =
		Paragraph::new(search_bar_text).alignment(Alignment::Left).block(untitled_ui_block());
	frame.render_widget(search_bar, chunks[1]);
}

/// Renders the search section.
pub fn render_search_section(frame: &mut Frame<'_>, size: Rect, search_term: Option<&str>) {
	render_search_bar_top_row(frame, search_section_layout().split(size)[0], search_term);
}
