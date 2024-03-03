//! A search results panel, displaying 10 results at a time.
//! Search results when lacking space will scroll.

use std::{
	fmt::format,
	time::{
		Duration,
		SystemTime,
		UNIX_EPOCH,
	},
};

use ansi_to_tui::IntoText;
use chrono::{
	DateTime,
	Utc,
};
use pluralizer::pluralize;
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
		Modifier,
		Style,
	},
	text::Text,
	widgets::{
		BorderType,
		Paragraph,
	},
	Frame,
};

use crate::{
	core::terminal::BackendType,
	games::{
		Game,
		GameMetadata,
	},
	ui::{
		components::{
			presets::{
				highlight_block,
				titled_ui_block,
			},
			scroll_tracker::ScrollTracker,
		},
		util::stylize_raw,
	},
};

#[must_use]
fn highlight_keyword(keyword: Option<&str>, word: &str) -> String {
	if keyword.is_none() {
		return word.to_string();
	}

	let keyword = keyword.unwrap();
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

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn play_status_text(metadata: &GameMetadata) -> String {
	let play_count = metadata.play_count();
	let last_played = metadata.last_played();
	if metadata.played() {
		let system_time = UNIX_EPOCH + Duration::from_secs(last_played.unwrap());
		let datetime = DateTime::<Utc>::from(system_time);
		let date_str = datetime.format("%d/%m/%Y");

		format!(
			"Played {} {}, last played at {}",
			stylize_raw(play_count),
			pluralize("time", play_count as isize, false),
			stylize_raw(date_str),
		)
	} else {
		stylize_raw("Never played before!")
	}
}

#[must_use]
fn search_result_text(search_term: Option<&str>, metadata: &GameMetadata) -> Text<'static> {
	format!(
		"{}: {}\n{}: {}\n{}: v{}\n{}",
		stylize_raw("Name"),
		highlight_keyword(search_term, metadata.name()),
		stylize_raw("Description"),
		highlight_keyword(search_term, metadata.description()),
		stylize_raw("Created at"),
		highlight_keyword(search_term, metadata.version_created()),
		play_status_text(metadata),
	)
	.into_text()
	.unwrap()
}

fn render_search_result(
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
	search_term: Option<&str>,
	result_index: u64,
	selected_index: Option<u64>,
	metadata: &GameMetadata,
) {
	let result_contents = search_result_text(search_term, metadata);
	let mut result_block = titled_ui_block(format!("[{}]", result_index + 1));
	if selected_index.map_or(false, |index| index == result_index) {
		result_block = highlight_block(result_block);
	}
	let result_paragraph = Paragraph::new(result_contents)
		.alignment(Alignment::Center)
		.block(result_block)
		.scroll((0, 0));
	frame.render_widget(result_paragraph, size);
}

#[must_use]
fn search_results_layout(count: u64) -> Layout {
	let mut constraints = vec![Constraint::Max(6); count as usize];
	constraints.push(Constraint::Max(0));

	Layout::default()
		.direction(Direction::Vertical)
		.vertical_margin(1)
		.horizontal_margin(2)
		.constraints(constraints)
}

/// Renders the search results.
pub fn render_search_results(
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
	search_term: Option<&str>,
	results: &[GameMetadata],
	scroll_tracker: &ScrollTracker,
) {
	frame.render_widget(titled_ui_block("Search Results"), size);
	let chunks = search_results_layout(scroll_tracker.range.unwrap()).split(size);
	for (index, metadata) in results.iter().enumerate() {
		render_search_result(
			frame,
			chunks[index],
			search_term,
			index as u64 + scroll_tracker.start,
			scroll_tracker.selected,
			metadata,
		);
	}
}
