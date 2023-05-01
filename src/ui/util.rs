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
use ratatui::text::{
	Spans,
	Text,
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
	get_mut_terminal().clear()?;
	Ok(())
}

/// ANSI-parses a [`Vec`] of [String] lines into individual [Text] objects.
/// This function is useful when making a list or a muliple blocks of text,
/// and you want stylizing done in those text.
pub fn ansi_parse_lines(lines: Vec<String>) -> Vec<Text<'static>> {
	lines.iter().map(String::into_text).map(Result::unwrap).collect()
}

/// Gets the version of the crate, or returns "version... not found :(" if one
/// was unable to be retrieved.
#[must_use]
pub fn get_crate_version() -> String {
	format!(
		"v{}",
		std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "ersion... not found :(".to_string())
	)
}
