//! Some utilities for working with the terminal.

#![allow(clippy::needless_pass_by_value)]

use ansi_to_tui::IntoText;
use crossterm::{
	execute,
	terminal::{
		Clear,
		ClearType,
	},
};
use ratatui::{
	layout::Alignment,
	text::{
		Spans,
		Text,
	},
	widgets::{
		Block,
		BorderType,
		Borders,
	},
};
use tiny_gradient::{
	Gradient,
	GradientStr,
};

use crate::core::{
	terminal::get_mut_terminal,
	Outcome,
};

/// [Disables raw mode](crossterm::terminal::disable_raw_mode), executes the
/// statements provided, and [enable raw
/// mode](crossterm::terminal::enable_raw_mode).
///
/// Note that this macro does make use of the `?`
/// operator to propagate errors in functions that expect a [Result] or a
/// [Result] equivalent.
#[macro_export]
macro_rules! disable_raw_mode {
	($($p:expr),*) => {
		crossterm::terminal::disable_raw_mode()?;
		$($p)*;
		crossterm::terminal::enable_raw_mode()?;
	};
}

/// Stylizes text with a gradient, converting them
/// into [`ratatui`]'s [Text] form for wider usage.
#[must_use]
pub fn stylize<T: ToString>(text: T) -> Text<'static> {
	stylize_raw(text).into_text().unwrap()
}

/// Stylizes text with a gradient.
#[must_use]
pub fn stylize_raw<T: ToString>(text: T) -> String {
	text.to_string().gradient(Gradient::Fruit).to_string()
}

/// Stylizes text with a gradient, immediately returning the first line.
#[must_use]
pub fn stylize_first<T: ToString>(text: T) -> Spans<'static> {
	stylize(text).lines[0].clone()
}

/// Clears the terminal.
pub fn clear_terminal() -> Outcome<()> {
	Ok(execute!(get_mut_terminal().backend_mut(), Clear(ClearType::All),)?)
}

/// The default [`ratatui`] block template.
pub fn ui_block<'a, T: ToString>(title: T) -> Block<'a> {
	Block::default()
		.borders(Borders::ALL)
		.border_type(BorderType::Rounded)
		.title_alignment(Alignment::Center)
		.title(stylize_first(title.to_string().as_str()))
}
