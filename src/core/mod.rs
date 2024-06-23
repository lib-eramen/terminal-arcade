//! # The `core` module
//!
//! This module contains some core functionality that all other functionalities
//! depend on.

use std::{
	path::{
		Path,
		PathBuf,
	},
	time::Duration,
};

use bool_toggle::Toggler;
use crossterm::{
	cursor::{
		DisableBlinking,
		EnableBlinking,
		Hide,
		MoveTo,
		Show,
	},
	event::{
		poll,
		read,
		DisableBracketedPaste,
		DisableFocusChange,
		DisableMouseCapture,
		EnableBracketedPaste,
		EnableFocusChange,
		EnableMouseCapture,
		Event,
		KeyCode,
		KeyEvent,
		KeyModifiers,
	},
	execute,
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
		EnterAlternateScreen,
		LeaveAlternateScreen,
	},
};
use ratatui::{
	layout::{
		Constraint,
		Layout,
	},
	style::{
		Color,
		Style,
	},
};

use self::terminal::get_terminal;

pub mod handler;
pub mod terminal;

/// The directory where Terminal Arcade saves all of its data.
/// NOT TO BE USED DIRECTLY. This path does not include the home dir.
/// Use [`get_save_dir`] for this instead.
pub const SAVE_DIR: &str = ".terminal-arcade";

/// Gets the save directory of Terminal Arcade.
/// Always use this function over the constant [`SAVE_DIR`].
#[must_use]
pub fn get_save_dir() -> PathBuf {
	home::home_dir().unwrap().as_path().to_owned().join(SAVE_DIR)
}

pub use handler::Handler;
