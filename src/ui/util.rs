//! Some utilities for working with the terminal.

#![allow(clippy::needless_pass_by_value)]

use std::time::{
	SystemTime,
	UNIX_EPOCH,
};

use ansi_to_tui::IntoText;
use crossterm::{
	execute,
	terminal::{
		Clear,
		ClearType,
	},
};
use ratatui::text::{
	Line,
	Text,
};
use tiny_gradient::{
	Gradient,
	GradientStr,
	RGB,
};

use crate::core::terminal::get_mut_terminal;

/// A list of colors used for the rainbow gradient.
static RAINBOW_GRADIENT_COLORS: [RGB; 6] = [
	RGB::new(255, 102, 99),  // red
	RGB::new(254, 177, 68),  // orange
	RGB::new(253, 253, 151), // yellow
	RGB::new(158, 224, 158), // green
	RGB::new(158, 193, 207), // blue
	RGB::new(204, 153, 201), // purple
];

fn get_gradient_colors() -> Vec<RGB> {
	let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
	let remainder = current_time % 900;
	let mut new_gradient = RAINBOW_GRADIENT_COLORS;
	new_gradient.rotate_right((remainder / 150) as usize);
	new_gradient[0..4].into()
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
	text.to_string().gradient(get_gradient_colors()).to_string()
}

/// Stylizes text with a gradient, immediately returning the first line.
#[must_use]
pub fn stylize_first<T: ToString>(text: T) -> Line<'static> {
	stylize(text).lines[0].clone()
}

/// Clears the terminal.
pub fn clear_terminal() -> anyhow::Result<()> {
	Ok(get_mut_terminal().clear()?)
}

/// ANSI-parses a [`Vec`] of [String] lines into individual [Text] objects.
/// This function is useful when making a list or a muliple blocks of text,
/// and you want stylizing done in those text.
pub fn ansi_parse_lines(lines: Vec<String>) -> Vec<Text<'static>> {
	lines.iter().map(String::into_text).map(Result::unwrap).collect()
}

/// Gets the version of the crate, or returns "NOT.FOUND" if one
/// was unable to be retrieved.
#[must_use]
pub fn get_crate_version() -> String {
	format!(
		"v{}",
		std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "NOT.FOUND".to_string())
	)
}
