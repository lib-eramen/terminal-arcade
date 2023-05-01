//! A module containing the [Screen] trait, a trait needed to, basically, do
//! everything on the terminal in Terminal Arcade. See the [Screen] trait to get
//! started. It also contains the various screens that the game uses to present
//! itself on the terminal.

use crossterm::event::Event;
use ratatui::Frame;

use crate::core::{
	terminal::BackendType,
	Outcome,
};

pub mod welcome;

/// The trait for handling drawing on the terminal and receiving events from the
/// user.
/// One should always start here when making a game/screen.
#[must_use]
pub trait Screen {
	/// Called when an event is passed along to the screen,
	/// possibly from [`crate::TerminalArcade`], but also from whatever screen
	/// spawned this screen.
	fn event(&mut self, _event: &Event) -> Outcome<()> {
		Ok(())
	}

	/// Called when the screen is being closed.
	/// This can be called when the entire application is being quitted (in the
	/// proper manner, of course, not through a crash or a panic).
	fn close(&mut self) -> Outcome<()> {
		Ok(())
	}

	/// Indicates that the screen is ready to be closed.
	/// If the screen is ready to be closed, the implementation of this function
	/// should return true. Otherwise, it should return false.
	fn is_closing(&self) -> bool;

	/// Indicates the screen that this screen itself is trying to create.
	/// If the window wants to create another screen, this function should
	/// return [Some], with the window inside it. Otherwise, return [None].
	fn screen_created(&self) -> Option<Box<dyn Screen>>;

	/// Paints the UI that the screen represent.
	/// This method is also called when a resize event is triggered.
	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>);

	/// The title for the [Handler] to change to.
	fn title(&self) -> &str;
}

pub use welcome::WelcomeScreen;
