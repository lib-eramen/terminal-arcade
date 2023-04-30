//! A collection of [`ratatui`] widget presets that I (the author and senior
//! engineer of this thing) think looks good. You, dear collaborator, can
//! propose a change via a PR, but unless it makes the UI look so *objetively*
//! better than without the change, this module shall stay as it is.

#![allow(clippy::needless_pass_by_value)]

use ratatui::{
	layout::Alignment,
	widgets::{
		Block,
		BorderType,
		Borders,
	},
};

use crate::ui::util::stylize_first;

/// The default [`ratatui`] block template, with a styled title.
#[must_use]
pub fn titled_ui_block<'a, T: ToString>(title: T) -> Block<'a> {
	untitled_ui_block()
		.title_alignment(Alignment::Center)
		.title(stylize_first(title.to_string().as_str()))
}

/// The default [`ratatui`] block template (no borders, but rounded border type
/// preset), untitled.
#[must_use]
pub fn untitled_ui_block<'a>() -> Block<'a> {
	Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
}
