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
	/// Returns the initial state that's associated with the screen.
	pub fn get_init_state(&self) -> ScreenState {
		todo!()
	}

	/// Closes the screen.
	pub fn close(&mut self, state: &mut ScreenState) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming [`Event`].
	pub fn event(
		&mut self,
		state: &mut ScreenState,
		event: &Event,
	) -> crate::Result<()> {
		todo!()
	}

	/// Renders this screen.
	pub fn render(
		&mut self,
		state: &mut ScreenState,
		frame: &mut Frame,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming [key event](KeyEvent).
	pub fn key(
		&mut self,
		state: &mut ScreenState,
		key: KeyEvent,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming [mouse event](MouseEvent).
	pub fn mouse(
		&mut self,
		state: &mut ScreenState,
		mouse: MouseEvent,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming paste.
	pub fn paste(
		&mut self,
		state: &mut ScreenState,
		text: String,
	) -> crate::Result<()> {
		todo!()
	}

	/// Handles an incoming [`FocusChange`].
	pub fn focus(
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
