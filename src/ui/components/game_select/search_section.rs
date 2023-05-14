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
	game::Game,
	ui::{
		components::ui_presets::{
			titled_ui_block,
			untitled_ui_block,
		},
		util::{
			stylize,
			stylize_raw,
		},
	},
};

/// Returns the text used in the back "button".
#[must_use]
pub fn back_button_text() -> Text<'static> {
	stylize("← Back ([Esc])")
}

/// Returns the default text used in the search bar.
#[must_use]
pub fn search_bar_default_text() -> Text<'static> {
	stylize(" 🔎︎ Search... ([Ctrl-D to clear]")
}

/// Returns the I'm Feeling Lucky help text.
#[must_use]
pub fn im_feeling_lucky_text() -> Text<'static> {
	let reset = Attribute::Reset;
	format!(
		r#"Feeling {}? {} for a {} game!{reset}
		Search page {}? {}? Use {} to adjust the density! ({}){reset}
		Feeling kind of annoyed with this popup taking space? Toggle with {}!{reset}"#,
		stylize_raw("lucky"),
		stylize_raw("[Ctrl-R]"),
		stylize_raw("random"),
		stylize_raw("too dense"),
		stylize_raw("Not dense enough"),
		stylize_raw("<- and ->"),
		stylize_raw("5 <= density <= 10"),
		stylize_raw("[Tab]"),
	)
	.into_text()
	.unwrap()
}

/// Returns the layout for a search bar.
#[must_use]
pub fn search_bar_layout() -> Layout {
	Layout::default().direction(Direction::Horizontal).margin(0).constraints([
		Constraint::Ratio(1, 7), // Back "button"
		Constraint::Ratio(6, 7), // Search area
	])
}

/// Returns the layout for the general search bar section.
#[must_use]
pub fn search_section_layout(display_help_text: bool) -> Layout {
	let mut constraints = vec![
		Constraint::Max(3), // Back "button" and search bar
		Constraint::Max(5), // Controls help text
		Constraint::Max(0), // Prevent blocks from taking up remaining space
	];
	if !display_help_text {
		constraints.remove(1);
	}

	Layout::default()
		.direction(Direction::Vertical)
		.vertical_margin(0)
		.horizontal_margin(0)
		.constraints(constraints)
}

/// Renders the top row of the search bar section.
pub fn render_search_bar_top_row(
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
	search_term: Option<&str>,
) {
	let chunks = search_bar_layout().split(size);

	let back_button =
		Paragraph::new(back_button_text()).alignment(Alignment::Center).block(untitled_ui_block());
	frame.render_widget(back_button, chunks[0]);

	let search_bar_text = search_term.map_or_else(search_bar_default_text, |term| {
		stylize(format!(" 🔎︎ {term}█"))
	});
	let search_bar =
		Paragraph::new(search_bar_text).alignment(Alignment::Left).block(untitled_ui_block());
	frame.render_widget(search_bar, chunks[1]);
}

fn render_search_bar_bottom_row(frame: &mut Frame<'_, BackendType>, size: Rect) {
	let bottom_row = Paragraph::new(im_feeling_lucky_text())
		.alignment(Alignment::Center)
		.block(titled_ui_block("Controls"));
	frame.render_widget(bottom_row, size);
}

/// Renders the search section.
pub fn render_search_section(
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
	search_term: Option<&str>,
	display_help_text: bool,
) {
	let chunks = search_section_layout(display_help_text).split(size);
	render_search_bar_top_row(frame, chunks[0], search_term);
	if display_help_text {
		render_search_bar_bottom_row(frame, chunks[1]);
	}
}
