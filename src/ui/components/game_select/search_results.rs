//! A search results panel, displaying 10 results at a time.
//! Search results when lacking space will scroll.

use std::time::{
	Duration,
	UNIX_EPOCH,
};

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
	ui::components::{
		presets::{
			highlight_block,
			titled_ui_block,
		},
		scroll_tracker::ScrollTracker,
	},
};

#[must_use]
fn highlight_keyword<'a>(keyword: Option<&'a str>, string: &'a str) -> Text<'a> {
	if keyword.is_none() {
		return Text::default();
	}

	let keyword = keyword.unwrap();
	let keyword_index = string.to_lowercase().find(&keyword.to_lowercase());
	if keyword_index.is_none() {
		return string.into();
	}

	let mut new_string = Text::default();
	let mut index = 0;
	while index < string.len() {
		if index + keyword.len() == string.len() {
			new_string.extend(Text::from(&string[index..]));
			break;
		}
		let current_word = &string[index..index + string.len()];
		if current_word == keyword {
			new_string.extend(Text::styled(
				keyword,
				Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
			));
			index += string.len();
		}
	}
	new_string
}
