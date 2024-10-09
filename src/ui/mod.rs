//! User interface structures in Terminal Arcade.

use crossterm::{
	event::{
		DisableMouseCapture,
		EnableMouseCapture,
	},
	execute,
};
use ratatui::{
	layout::Rect,
	Frame,
};
use serde::{
	Deserialize,
	Serialize,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
	events::{
		AppEvent,
		Event,
		InputEvent,
		ScreenEvent,
	},
	tui::Terminal,
	ui::screens::{
		Screen,
		ScreenHandle,
	},
};

pub mod screens;
pub mod widgets;

/// A UI element that renders and receives events.
pub trait UiElement {
	type State;

	/// Handles an incoming [`Event`].
	fn event(&mut self, state: Self::State, event: Event) -> crate::Result<()>;

	/// Renders this element.
	fn render(&self, state: Self::State, frame: &mut Frame<'_>, size: Rect);
}

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

	/// Screens that this UI handles.
	/// The top most screen (last element) is named the "active" screen.
	/// It gets to render and receive events.
	screens: Vec<ScreenHandle>,

	/// Event channel.
	event_sender: UnboundedSender<Event>,
}

impl Ui {
	/// Constructs an empty UI.
	pub fn new(event_sender: UnboundedSender<Event>) -> Self {
		Self {
			run_state: UiRunState::Running,
			screens: Vec::new(),
			event_sender,
		}
	}

	/// [`debug_assert`]s that there are screens.
	fn assert_screens_nonemptiness(&self) {
		debug_assert!(!self.is_empty(), "no screens left in stack");
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

	/// Handles a [`Terminal`]-related [`Event`]. On an [`AppEvent::Render`]
	/// event, a [`CompletedFrame`] is returned.
	#[expect(clippy::unwrap_used)]
	pub fn handle_terminal_event(
		&mut self,
		terminal: &mut Terminal,
		event: &Event,
	) -> std::io::Result<()> {
		self.assert_screens_nonemptiness();
		match event {
			Event::Input(InputEvent::ResizeTerminal(w, h)) => {
				terminal.resize(Rect::new(0, 0, *w, *h))
			},
			Event::App(AppEvent::Render) => {
				let _completed_frame = terminal.draw(|frame| {
					self.get_active_screen()
						.unwrap()
						.render(frame, frame.size());
				})?;
				Ok(())
			},
			_ => Ok(()),
		}
	}

	/// Handles an incoming [`Event`].
	#[expect(clippy::unwrap_used)]
	pub fn event(
		&mut self,
		terminal: &mut Terminal,
		event: Event,
	) -> crate::Result<()> {
		self.assert_screens_nonemptiness();
		self.handle_terminal_event(terminal, &event)?;
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

	/// Gets a reference to the current active screen.
	pub fn get_active_screen(&mut self) -> Option<&ScreenHandle> {
		self.screens.last()
	}

	/// Gets a mutable reference to the current active screen.
	pub fn get_mut_active_screen(&mut self) -> Option<&mut ScreenHandle> {
		self.screens.last_mut()
	}

	/// Constructs a new screen as active and clones the UI's own sender for
	/// the new screen's use.
	pub fn push_active_screen<S>(&mut self, screen: S) -> crate::Result<()>
	where
		S: Screen + 'static,
	{
		let handle = ScreenHandle::new(screen, self.event_sender.clone())?;
		Self::enable_mouse_conditionally(handle.state.captures_mouse)?;
		self.screens.push(handle);
		Ok(())
	}

	/// Enables mouse capture on condition.
	fn enable_mouse_conditionally(
		captures_mouse: bool,
	) -> Result<(), std::io::Error> {
		if captures_mouse {
			execute!(std::io::stdout(), EnableMouseCapture)
		} else {
			execute!(std::io::stdout(), DisableMouseCapture)
		}
	}

	/// Pops the active screen, returning an error if there is none left.
	pub fn pop_active_screen(&mut self) -> Option<ScreenHandle> {
		self.screens.pop()
	}
}
