//! Handler to manage screens and rendering them.

use crossterm::event::{
	KeyEvent,
	MouseEvent,
};
use ratatui::Frame;
use serde::{
	Deserialize,
	Serialize,
};

use crate::{
	event::Event,
	tui::FocusChange,
};

pub mod handler;
pub mod state;

/// A screen that holds state, receives events and renders to the terminal.
#[derive(Debug, Serialize, Deserialize)]
pub enum Screen {}

impl Screen {
	/// Returns the initial metadata that's associated with the screen.
	fn init_metadata(&self) -> ScreenState {
		todo!()
	}

	/// Handles an incoming [`Event`].
	fn event(
		&mut self,
		state: &mut ScreenState,
		event: &Event,
	) -> crate::Result<()> {
		todo!()
	}

	/// Renders this screen's UI to the terminal.
	fn render(
		&mut self,
		frame: &mut Frame,
		state: &mut ScreenState,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming [key event](KeyEvent).
	fn key(
		&mut self,
		state: &mut ScreenState,
		key: KeyEvent,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming [mouse event](MouseEvent).
	fn mouse(
		&mut self,
		state: &mut ScreenState,
		mouse: MouseEvent,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming paste.
	fn paste(
		&mut self,
		state: &mut ScreenState,
		text: String,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming focus change.
	fn focus(
		&mut self,
		state: &mut ScreenState,
		change: FocusChange,
	) -> crate::Result<()> {
		todo!()
	}
}

pub use state::{
	ScreenHandle,
	ScreenState,
};
