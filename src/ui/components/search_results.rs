//! A search results panel, displaying 10 results at a time.
//! Search results when lacking space will scroll.

use std::fmt::format;

use ansi_to_tui::IntoText;
use ratatui::{
	layout::{
		Layout,
		Rect, Direction, Constraint, Alignment,
	},
	Frame, widgets::Paragraph,
};

use super::ui_presets::titled_ui_block;
use crate::{
	core::terminal::BackendType,
	game::{Game, GameMetadata}, ui::util::stylize_raw,
};

/// Highlights the keyword in the word (ANSI-colors the keyword substring in it).
#[must_use]
pub fn highlight_keyword(keyword: &str, word: &str)	-> String {
	let keyword_index = word.to_lowercase().find::<&str>(keyword.to_lowercase().as_ref());
	if keyword_index.is_none() {
		return word.to_string();
	}
	
	let keyword_index = keyword_index.unwrap();
	let highlighted_range = keyword_index..(keyword_index + keyword.len());
	let new_string = format!(
		"{}{}{}",
		&word[..keyword_index],
		stylize_raw(&word[highlighted_range]),
		&word[(keyword_index + keyword.len())..]
	);
	new_string
}

/// Render a search result.
pub fn render_search_result(
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
	search_term: Option<&str>,
	result_index: usize,
	metadata: &GameMetadata,
) {
	let game_name = if let Some(term) = search_term {
		highlight_keyword(term, metadata.name())
	} else {
		metadata.name().to_string()
	};
	let result_contents = format!(
		"{}: {}\n{}: {}\n{}: {}, {}: v{}",
		stylize_raw("Name"), game_name,
		stylize_raw("Description"), metadata.description(),
		stylize_raw("Made by"), metadata.authors_string(),
		stylize_raw("created at"), metadata.version_created(),
	).into_text().unwrap();
	let result_paragraph = Paragraph::new(result_contents)
		.alignment(Alignment::Center)
		.block(titled_ui_block(format!("[{result_index}]")))
		.scroll((0, 0));
	frame.render_widget(result_paragraph, size);
}

/// Returns the layout of the search results.
#[must_use]
pub fn search_results_layout() -> Layout {
	let mut constraints = vec![Constraint::Max(5); 7];
	constraints.push(Constraint::Max(0));

    Layout::default()
		.direction(Direction::Vertical)
		.vertical_margin(2)
		.horizontal_margin(3)
		.constraints(constraints)
}

/// Renders the search results.
pub fn render_search_results(
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
	search_term: Option<&str>,
	results: &[GameMetadata],
) {
	frame.render_widget(titled_ui_block("Search Results"), size);

	let chunks = search_results_layout().split(size);
	for (index, metadata) in results.iter().enumerate() {
		render_search_result(
			frame,
			chunks[index],
			search_term, 
			index,
			metadata,
		);
	}
}