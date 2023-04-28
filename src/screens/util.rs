//! Some utilities for working with the terminal.

use crossterm::{
	execute,
	style::Attribute,
	terminal::{
		Clear,
		ClearType,
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

/// Highlights text as bold and colors them with a gradient.
/// Note that you might need to reset the text after applying the bold
/// attribute.
#[must_use]
pub fn highlight(text: &str) -> String {
	format!("{}{}", Attribute::Bold, text.gradient(Gradient::Fruit))
}

/// Clears the terminal.
pub fn clear_terminal() -> Outcome<()> {
	Ok(execute!(get_mut_terminal().backend_mut(), Clear(ClearType::All),)?)
}
