//! Some utilities for working with the terminal.

#![allow(clippy::needless_pass_by_value)]

use std::time::{
	SystemTime,
	UNIX_EPOCH,
};

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

/// Gets the version of the crate, or returns "NOT.FOUND" if one
/// was unable to be retrieved.
/// TODO: Move to footer where this information is used
#[must_use]
pub fn get_crate_version() -> String {
	format!(
		"v{}",
		std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "NOT.FOUND".to_string())
	)
}
