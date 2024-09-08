//! Handles the state of the application, including the actual data as well as
//! rendering of it.
//!
//! Note that the terms "close" and "quit" aren't synonyms the way they are used
//! here - "quitting" implies that the exits immediately (in other words,
//! force-quit), while "closing" doesn't.

use color_eyre::eyre::eyre;
use derive_new::new;
use ratatui::layout::Rect;
use serde::Serialize;
use tokio::sync::mpsc::error::TryRecvError;
use tracing::{
	debug,
	error,
	info,
	instrument,
};

use crate::{
	components::screens::home::HomeScreen,
	config::Config,
	events::{
		tui::{
			FocusChange,
			InputEvent,
		},
		AppEvent,
		Event,
		TuiEvent,
	},
	tui::Tui,
	ui::{
		screens::ScreenHandler,
		UiRunState,
	},
	utils::UnboundedChannel,
};

/// Running state of the application.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[allow(missing_docs)] // Relatively obvious variant names
enum AppRunState {
	/// The app has not started.
	#[default]
	Pending,

	/// The app is running.
	Running,

	/// The app is closing (not forcibly).
	Closing,

	/// The app is quitting (forcibly).
	Quitting,

	/// The app has finished.
	Finished,
}

/// Handler for application state and rendering.
#[derive(Debug, Serialize, new)]
pub struct App {
	/// Running state of the application.
	#[serde(skip)]
	run_state: AppRunState,

	/// Screen handler.
	screen_handler: ScreenHandler,

	/// [`Event`] channel. The sender of this channel is cloned for screens to
	/// send their own events to the app.
	event_channel: UnboundedChannel<Event>,

	/// Buffer for accumulating input events per tick.
	#[serde(skip)]
	input_buffer: Vec<InputEvent>,
}

impl Default for App {
	fn default() -> Self {
		let event_channel = UnboundedChannel::new();
		Self::new(
			AppRunState::default(),
			ScreenHandler::new(event_channel.get_sender().clone()),
			event_channel,
			Vec::default(),
		)
	}
}

impl App {
	/// Starts the app with the provided terminal interface and with a landing
	/// [`HomeScreen`].
	#[instrument(name = "run-app", skip_all)]
	pub fn run(mut self, mut tui: Tui, config: Config) -> crate::Result<()> {
		debug!(?config, "using provided config");
		self.set_run_state(AppRunState::Running);
		tui.enter()?;
		self.screen_handler.push_active_screen(HomeScreen);
		self.event_loop(tui)?;
		Ok(())
	}

	/// App event loop.
	fn event_loop(&mut self, mut tui: Tui) -> crate::Result<()> {
		loop {
			self.relay_tui_event(&mut tui)?;
			self.process_app_events(&mut tui)?;
			self.update()?;
			if self.run_state == AppRunState::Finished {
				break;
			}
		}
		Ok(())
	}

	/// Handles an event received from from the provided [`Tui`], then
	/// sends that event through the [channel].
	fn relay_tui_event(&mut self, tui: &mut Tui) -> crate::Result<()> {
		let event = match tui.try_recv_event() {
			Ok(event) => event,
			Err(err) => return Self::handle_try_recv_err(err, "tui"),
		};
		self.handle_tui_event(event)
	}

	/// Handles an error encountered while
	/// [`try_recv`](tokio::sync::mpsc::UnboundedReceiver::try_recv)ing from a
	/// particular event source. The source name is interpolated directly
	/// into the log & error messages.
	///
	/// The function returns something the event loop can propagate
	/// back up and handle accordingly. Only a [`TryRecvError::Disconnected`]
	/// will result in an error.
	fn handle_try_recv_err(
		err: TryRecvError,
		source: &'static str,
	) -> crate::Result<()> {
		match err {
			TryRecvError::Empty => {
				info!("{source} is currently empty");
				Ok(())
			},
			TryRecvError::Disconnected => Err(eyre!(
				"while trying to receive from the {source}: {source} event \
				 channel disconnected"
			)),
		}
	}

	/// Handles a given [`TuiEvent`].
	fn handle_tui_event(&mut self, event: TuiEvent) -> crate::Result<()> {
		match event {
			TuiEvent::Hello => {
				info!("received init event! very considerate of you, kind tui");
			},
			TuiEvent::Tick if !self.input_buffer.is_empty() => {
				let events = self.input_buffer.drain(..).collect();
				self.event_channel
					.send(Event::App(AppEvent::UserInputs(events)))?;
			},
			TuiEvent::Input(input) => self.input_buffer.push(input),
			event => {
				self.event_channel
					.send(Event::App(event.try_into().unwrap()))?;
			},
		}
		Ok(())
	}

	/// Receives and handles all incoming events from the [event
	/// channel](Self::event_channel).
	fn process_app_events(&mut self, tui: &mut Tui) -> crate::Result<()> {
		loop {
			match self.event_channel.try_recv() {
				Ok(event) => self.event(tui, &event)?,
				Err(err) => return Self::handle_try_recv_err(err, "app"),
			}
		}
	}

	/// Returns whether the app has reached a point where it can be declared
	/// [finished](AppRunState::Finished).
	fn can_be_finished(&self) -> bool {
		self.run_state == AppRunState::Quitting
			|| self.screen_handler.get_run_state() == UiRunState::Finished
	}

	/// Sets the run state of the app.
	fn set_run_state(&mut self, run_state: AppRunState) {
		info!(?run_state, "setting app run state");
		self.run_state = run_state;
	}

	/// Updates the app. Note that this has nothing to do with the tick of the
	/// app.
	fn update(&mut self) -> crate::Result<()> {
		self.screen_handler.update()?;
		if self.can_be_finished() {
			self.set_run_state(AppRunState::Finished);
		}
		Ok(())
	}

	/// Handles a given event.
	fn event(&mut self, tui: &mut Tui, event: &Event) -> crate::Result<()> {
		if event.should_be_logged() {
			info!(?event, "receiving event");
		}
		if let Event::App(ref app_event) = event {
			self.prehandle_app_event(tui, app_event)?;
		}
		self.screen_handler.event(event)
	}

	/// Handles an app event before forwarding it to the active screen for
	/// handling.
	fn prehandle_app_event(
		&mut self,
		tui: &mut Tui,
		app_event: &AppEvent,
	) -> crate::Result<()> {
		match app_event {
			AppEvent::Tick => self.tick(),
			AppEvent::Render => self.render(tui),
			AppEvent::CloseApp => {
				self.close_app();
				Ok(())
			},
			AppEvent::QuitApp => {
				self.quit_app();
				Ok(())
			},
			AppEvent::CloseActiveScreen => self.close_active_screen(),
			AppEvent::ErrorOccurred(msg) => self.error_occurred(msg),
			AppEvent::ChangeFocus(change) => self.change_focus(*change),
			AppEvent::ResizeTerminal(w, h) => self.resize_terminal(tui, *w, *h),
			AppEvent::PasteText(_) | AppEvent::UserInputs(_) => Ok(()),
		}
	}

	/// Advances through one tick. The function sends a [`Keys`]
	/// (`AppEvent::Keys`) event with the [`KeyEvent`]s [`drain`](Vec::drain)ed
	/// from [`Self::tick_key_events`].
	fn tick(&mut self) -> crate::Result<()> {
		if !self.input_buffer.is_empty() {
			let events = self.input_buffer.drain(..).collect();
			self.event_channel
				.send(Event::App(AppEvent::UserInputs(events)))?;
		}
		Ok(())
	}

	/// Renders the active screen.
	fn render(&mut self, tui: &mut Tui) -> crate::Result<()> {
		self.screen_handler.render(&mut tui.get_frame())
	}

	/// Closes the app.
	fn close_app(&mut self) {
		self.set_run_state(AppRunState::Closing);
	}

	/// Quits the app.
	fn quit_app(&mut self) {
		self.set_run_state(AppRunState::Quitting);
		self.screen_handler.clear_screens();
	}

	/// Closes the active screen.
	fn close_active_screen(&mut self) -> crate::Result<()> {
		self.screen_handler.pop_active_screen()?;
		Ok(())
	}

	/// Logs the error and displays it on a popup in the terminal.
	fn error_occurred(&mut self, msg: &str) -> crate::Result<()> {
		error!(msg, "an error event occurred");
		todo!();
	}

	/// Resizes the terminal and sends a [render](AppEvent::Render) event to
	/// re-render.
	fn resize_terminal(
		&mut self,
		tui: &mut Tui,
		w: u16,
		h: u16,
	) -> crate::Result<()> {
		tui.resize(Rect::new(0, 0, w, h))?;
		self.event_channel.send(Event::App(AppEvent::Render))?;
		Ok(())
	}

	/// Handles an focus change event.
	fn change_focus(&mut self, _change: FocusChange) -> crate::Result<()> {
		todo!()
	}
}
