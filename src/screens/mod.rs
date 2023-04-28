//! A module containing the [Screen] trait, a trait needed to, basically, do
//! everything on the terminal in Terminal Arcade. See the [Screen] trait to get
//! started. It also contains the various screens that the game uses to present
//! itself on the terminal.

use crossterm::event::Event;

use crate::core::Outcome;

pub mod util;
pub mod welcome;

/// The level of indentation to be used for printing. This is 8 spaces.
/// This static variable is intended to be replaced with a configurable
/// indentation option. TODO: Configuration option for indent.
pub static INDENT: &str = r#"        "#;

/// The trait for handling drawing on the terminal and receiving events from the
/// user.
/// One should always start here when making a game/screen.
#[must_use]
pub trait Screen {
	/// Called when the screen is first constructed, or "spawned". All
	/// initialization should go here.
	fn on_spawn(&mut self) -> Outcome<()>;

	/// Called when an event is passed along to the screen,
	/// possibly from [`crate::TerminalArcade`], but also from whatever screen
	/// spawned this screen.
	fn on_event(&mut self, event: &Event) -> Outcome<()>;

	/// Called when the screen is being closed.
	/// This can be called when the entire application is being quitted (in the
	/// proper manner, of course, not through a crash or a panic).
	fn on_close(&mut self) -> Outcome<()>;
}

pub use welcome::WelcomeScreen;
