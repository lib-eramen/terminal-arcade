//! Helper module for easier handling of the terminal.

use std::{
	borrow::BorrowMut,
	io::Stdout,
	sync::{
		Mutex,
		MutexGuard,
	},
};

use once_cell::sync::OnceCell;
use ratatui::{
	backend::CrosstermBackend,
	Terminal,
};

/// The type of [`ratatui`] backend used in Terminal Arcade.
pub type BackendType = CrosstermBackend<Stdout>;

/// The type of the terminal used in Terminal Arcade.
pub type ArcadeTerminal = Terminal<BackendType>;

/// The global terminal instance, shared by all mechanisms in Terminal Arcade.
/// See [`get_terminal`] for a more convenient way to access this static
/// variable, as it is wrapped under a layer of [`OnceCell`].
pub static TERMINAL: OnceCell<Mutex<ArcadeTerminal>> = OnceCell::new();

/// Creates the terminal for use in Terminal Arcade.
#[must_use]
pub fn create_terminal() -> ArcadeTerminal {
	let stdout = std::io::stdout();
	let backend = CrosstermBackend::new(stdout);
	ArcadeTerminal::new(backend).unwrap()
}

/// Initializes the terminal for use in Termianl Arcade.
pub fn initialize_terminal() {
	TERMINAL
		.set(Mutex::new(create_terminal()))
		.expect("Terminal should not have already been initialized");
}

/// Helper function for accessing the global terminal handle used throughout
/// Terminal Arcade.
#[must_use]
pub fn get_terminal() -> MutexGuard<'static, ArcadeTerminal> {
	TERMINAL.get().unwrap().lock().unwrap()
}
