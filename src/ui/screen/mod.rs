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
pub mod metadata;

/// A screen that holds state, receives events and renders to the terminal.
#[derive(Debug, Serialize, Deserialize)]
pub enum Screen {}

impl Screen {
	/// Handles an incoming [`Event`].
	fn handle_event(
		&mut self,
		state: &mut ScreenState,
		event: Event,
	) -> crate::Result<()> {
		todo!()
	}

	/// Renders this screen's UI to the terminal.
	fn render_ui(
		&mut self,
		frame: &mut Frame,
		state: &mut ScreenState,
	) -> crate::Result<()> {
		todo!()
	}

	/// Returns the initial metadata that's associated with the screen.
	fn init_metadata(&self) -> ScreenState {
		todo!()
	}

	/// Handles an incoming [key event](KeyEvent).
	fn handle_key_event(
		&mut self,
		state: &mut ScreenState,
		key: KeyEvent,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming [mouse event](MouseEvent).
	fn handle_mouse_event(
		&mut self,
		state: &mut ScreenState,
		mouse: MouseEvent,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming paste.
	fn handle_paste(
		&mut self,
		state: &mut ScreenState,
		text: String,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming focus change.
	fn handle_focus_change(
		&mut self,
		state: &mut ScreenState,
		focus: FocusChange,
	) -> crate::Result<()> {
		todo!()
	}
}

pub use metadata::{
	ScreenState,
	ScreenWithState,
};
