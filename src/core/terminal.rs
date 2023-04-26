//! Helper module for easier handling of the terminal.

use std::{
	cell::OnceCell,
	io::Stdout,
};

use tui::{
	backend::CrosstermBackend,
	Terminal,
};

/// The type of the terminal used in Terminal Arcade.
pub type TerminalType = Terminal<CrosstermBackend<Stdout>>;

/// The global terminal instance, shared by all mechanisms in Terminal Arcade.
/// See [`get_terminal`] for a more convenient way to access this static
/// variable, as it is wrapped under a layer of [`OnceCell`].
pub static mut TERMINAL: OnceCell<TerminalType> = OnceCell::new();

/// Initializes the terminal.
#[must_use]
pub fn initialize_terminal() -> TerminalType {
	let stdout = std::io::stdout();
	let backend = CrosstermBackend::new(stdout);
	Terminal::new(backend).unwrap()
}

/// Helper function for accessing the global terminal handle used throughout
/// Terminal Arcade.
#[must_use]
pub fn get_terminal() -> &'static TerminalType {
	unsafe { TERMINAL.get_or_init(initialize_terminal) }
}

/// Helper function for accessing the global terminal handle mutably used
/// throughout Terminal Arcade.
/// Note that if [`TERMINAL`] is not initialized, the function will panic.
#[must_use]
pub fn get_mut_terminal() -> &'static mut TerminalType {
	unsafe { TERMINAL.get_mut().unwrap() }
}
