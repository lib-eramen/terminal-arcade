//! Handler to manage screens and rendering them.

use color_eyre::eyre::eyre;
use ratatui::Frame;
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::{
		Event,
		ScreenEvent,
	},
	ui::{
		screens::{
			Screen,
			ScreenHandle,
		},
		UiRunState,
	},
};

/// Handler for screens. This struct dereferenes to the inner
/// [`Vec`] of [`ScreenHandle`]s, where the top screen is named
/// "active" and should be the one rendered and receiving events.
#[derive(Debug, Serialize)]
pub struct ScreenHandler {
	/// Running state of the screen handler.
	#[serde(skip)]
	run_state: UiRunState,

	/// Screens that this handler, well, handles.
	/// The top most screen (last element) is named the "active" screen.
	/// It gets to render and receive events.
	screens: Vec<ScreenHandle>,

	/// Event channel.
	#[serde(skip)]
	event_sender: UnboundedSender<Event>,
}

impl ScreenHandler {
	/// Constructs an empty screen handler.
	pub fn new(event_sender: UnboundedSender<Event>) -> Self {
		Self {
			run_state: UiRunState::Running,
			screens: Vec::new(),
			event_sender,
		}
	}

	/// Renders the screens this handler is holding to the terminal.
	/// If there are no active screens, nothing is rendered.
	pub fn render(&mut self, frame: &mut Frame) -> crate::Result<()> {
		if let Some(handle) = self.get_mut_active_screen() {
			handle.render(frame)?;
		}
		Ok(())
	}

	/// Updates the screen handler.
	///
	/// This method returns the screen that was closed, if there was one.
	/// Note that this method closes at most one screen every time it is called.
	///
	/// [running]: UiRunState::Running
	/// [closing]: UiRunState::Closing
	/// [finished]: UiRunState::Finished
	#[allow(clippy::unwrap_used, reason = "infallible")]
	pub fn update(&mut self) -> crate::Result<Option<ScreenHandle>> {
		if self.is_empty() {
			self.run_state = UiRunState::Finished;
			return Ok(None);
		}
		let handler_run_state = self.run_state;
		let active_screen = self.get_mut_active_screen().unwrap();
		match (handler_run_state, active_screen.state.run_state) {
			(UiRunState::Finished, _) => {},
			(_, UiRunState::Finished) => return Ok(self.pop_active_screen()),
			(_, UiRunState::Closing) => todo!(),
			(UiRunState::Running, UiRunState::Running) => todo!(),
			(UiRunState::Closing, UiRunState::Running) => {
				self.event_sender.send(Event::Screen(ScreenEvent::Close))?;
			},
		}
		Ok(None)
	}

	/// Handles an incoming [`Event`].
	pub fn event(&mut self, event: &Event) -> crate::Result<()> {
		if let Some(screen) = self.get_mut_active_screen() {
			screen.event(event)
		} else {
			Err(eyre!("no screens left in stack to receive events"))
		}
	}

	/// Sets the [run state](Self::run_state) to
	/// [`Closing`](UiRunState::Closing).
	pub fn close(&mut self) {
		self.run_state = UiRunState::Closing;
	}

	/// Performs actions to quit. (clears all of its screens, etc.)
	pub fn quit(&mut self) {
		self.clear_screens();
		self.run_state = UiRunState::Finished;
	}

	/// Returns the [run state](UiRunState) of the screen handler.
	pub fn get_run_state(&self) -> UiRunState {
		self.run_state
	}

	/// Checks if the screen handler doesn't have any more screens.
	pub fn is_empty(&self) -> bool {
		self.screens.is_empty()
	}

	/// Clears all screens from this screen handler.
	pub fn clear_screens(&mut self) {
		self.screens.clear();
	}

	/// Gets the current active screen (the final one on the
	/// [stack](Self::stack)) mutably.
	pub fn get_mut_active_screen(&mut self) -> Option<&mut ScreenHandle> {
		self.screens.last_mut()
	}

	/// Creates a new screen as active and clones the handler's own sender for
	/// the screen handle's use.
	pub fn push_active_screen<S>(&mut self, screen: S)
	where
		S: Screen + 'static,
	{
		self.screens
			.push(ScreenHandle::new(screen, self.event_sender.clone()));
	}

	/// Pops the active screen, returning an error if there is none left.
	pub fn pop_active_screen(&mut self) -> Option<ScreenHandle> {
		self.screens.pop()
	}
}
