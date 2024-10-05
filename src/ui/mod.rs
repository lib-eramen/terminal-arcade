//! User interface structures in Terminal Arcade.

use std::{
	cell::RefCell,
	rc::Rc,
};

use ratatui::Frame;
use serde::{
	Deserialize,
	Serialize,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::{
		Event,
		InputEvent,
		ScreenEvent,
	},
	tui::Terminal,
	ui::screens::{
		state::ScreenStateBuilderError,
		Screen,
		ScreenHandle,
	},
};

pub mod screens;
pub mod widgets;

/// Running state of any given UI moving part (a screen, widget) that runs and
/// closes.
///
/// "Close" here is used with a nuance - some other code is run or action is
/// expected to be done before the part is ready to be closed. As such,
/// "closing" does not apply when the part is being forcibly quit.
#[derive(
	Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum UiRunState {
	/// The part is running.
	#[default]
	Running,

	/// The part is closing and is not forced to immediately quit.
	/// The associated boolean indicates whether the
	Closing,

	/// The part has finished closing.
	Finished,
}

/// The UI of the app. This struct handles the screens
#[derive(Debug)]
pub struct Ui {
	/// Running state.
	run_state: UiRunState,

	/// Handle to the terminal.
	terminal: Rc<RefCell<Terminal>>,

	/// Screens that this UI handles.
	/// The top most screen (last element) is named the "active" screen.
	/// It gets to render and receive events.
	screens: Vec<ScreenHandle>,

	/// Event channel.
	event_sender: UnboundedSender<Event>,
}

impl Ui {
	/// Constructs an empty UI.
	pub fn new(
		terminal: Rc<RefCell<Terminal>>,
		event_sender: UnboundedSender<Event>,
	) -> Self {
		Self {
			run_state: UiRunState::Running,
			screens: Vec::new(),
			terminal,
			event_sender,
		}
	}

	/// Renders the screens this UI is holding to the terminal.
	/// If there are no active screens, nothing is rendered.
	pub fn render(&mut self, frame: &mut Frame) -> crate::Result<()> {
		if let Some(handle) = self.get_mut_active_screen() {
			handle.render(frame)?;
		}
		Ok(())
	}

	/// Sets the UI's run state to [`UiRunState::Finished`] if there
	/// are no more screens. Also returns the result of said predicate.
	fn finish_if_empty(&mut self) -> bool {
		let emptiness = self.is_empty();
		if emptiness {
			self.run_state = UiRunState::Finished;
		}
		emptiness
	}

	/// Updates the UI.
	///
	/// This method returns the screen that was closed, if there was one.
	/// Note that this method closes at most one screen every time it is called.
	///
	/// [running]: UiRunState::Running
	/// [closing]: UiRunState::Closing
	/// [finished]: UiRunState::Finished
	#[expect(clippy::unwrap_used, reason = "infallible")]
	pub fn update(&mut self) -> crate::Result<Option<ScreenHandle>> {
		if self.finish_if_empty() {
			return Ok(None);
		}
		let ui_run_state = self.run_state;
		let active_screen = self.get_mut_active_screen().unwrap();
		match (ui_run_state, active_screen.state.run_state) {
			(_, UiRunState::Finished) => {
				let finished_screen = self.pop_active_screen();
				self.finish_if_empty();
				return Ok(finished_screen);
			},
			(_, UiRunState::Closing)
			| (UiRunState::Running, UiRunState::Running) => {
				active_screen.update()?;
			},
			(UiRunState::Closing, UiRunState::Running) => {
				self.event_sender.send(ScreenEvent::Close.into())?;
			},
			(UiRunState::Finished, _) => {},
		}
		Ok(None)
	}

	/// Handles an incoming [`Event`].
	#[expect(clippy::unwrap_used)]
	pub fn event(&mut self, event: Event) -> crate::Result<()> {
		debug_assert!(
			!self.is_empty(),
			"no screens left in stack to receive events"
		);
		if let Event::Input(InputEvent::ResizeTerminal(..)) = event {
			self.terminal.borrow_mut().autoresize()?;
		}
		self.get_mut_active_screen().unwrap().event(event)
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

	/// Returns the [run state](UiRunState) of the UI.
	pub fn get_run_state(&self) -> UiRunState {
		self.run_state
	}

	/// Checks if the UI doesn't have any more screens.
	pub fn is_empty(&self) -> bool {
		self.screens.is_empty()
	}

	/// Clears all screens from this UI.
	pub fn clear_screens(&mut self) {
		self.screens.clear();
	}

	/// Gets the current active screen (the final one on the
	/// [stack](Self::stack)) mutably.
	pub fn get_mut_active_screen(&mut self) -> Option<&mut ScreenHandle> {
		self.screens.last_mut()
	}

	/// Constructs a new screen as active and clones the UI's own sender for
	/// the new screen's use.
	pub fn push_active_screen<S>(
		&mut self,
		screen: S,
	) -> Result<(), ScreenStateBuilderError>
	where
		S: Screen + 'static,
	{
		self.screens
			.push(ScreenHandle::new(screen, self.event_sender.clone())?);
		Ok(())
	}

	/// Pops the active screen, returning an error if there is none left.
	pub fn pop_active_screen(&mut self) -> Option<ScreenHandle> {
		self.screens.pop()
	}
}
