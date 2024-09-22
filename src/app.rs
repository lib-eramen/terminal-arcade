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
	config::Config,
	events::{
		input::InputEvent,
		AppEvent,
		Event,
		TuiEvent,
	},
	tui::Tui,
	ui::{
		Ui,
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

	/// UI of the app.
	screen_handler: Ui,

	/// [`Event`] channel. The sender of this channel is cloned for screens to
	/// send their own events to the app.
	event_channel: UnboundedChannel<Event>,
}

impl Default for App {
	fn default() -> Self {
		let event_channel = UnboundedChannel::new();
		Self::new(
			AppRunState::default(),
			Ui::new(event_channel.get_sender().clone()),
			event_channel,
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
		// self.screen_handler.push_active_screen(HomeScreen);
		self.event_loop(tui)?;
		Ok(())
	}

	/// App event loop.
	fn event_loop(&mut self, mut tui: Tui) -> crate::Result<()> {
		loop {
			self.relay_tui_event(&mut tui)?;
			self.process_all_events(&mut tui)?;
			self.update()?;
			if self.run_state == AppRunState::Finished {
				break;
			}
		}
		Ok(())
	}

	#[allow(clippy::expect_used, reason = "infallible")]
	/// Handles a given [`TuiEvent`], returning an [`AppEvent`] if applicable.
	pub fn handle_tui_event(event: TuiEvent) -> Option<AppEvent> {
		match event {
			TuiEvent::Hello => {
				info!(
					"i don't know what ur saying but hello to you! very \
					 considerate of you, kind tui"
				);
				None
			},
			event => Some(
				event
					.try_into()
					.expect("could not convert tui event into app event"),
			),
		}
	}

	/// Handles an event received from from the provided [`Tui`], transforms the
	/// event with the [middleman](TuiAppMiddleman), then sends the resulting
	/// [`AppEvent`] through the [channel], if there is any.
	fn relay_tui_event(&mut self, tui: &mut Tui) -> crate::Result<()> {
		let tui_event = match tui.try_recv_event() {
			Ok(event) => event,
			Err(err) => return Self::handle_try_recv_err(err, "tui"),
		};
		if let Some(app_event) = Self::handle_tui_event(tui_event) {
			self.event_channel.send(Event::App(app_event))?;
		}
		Ok(())
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
			TryRecvError::Empty => Ok(()),
			TryRecvError::Disconnected => Err(eyre!(
				"while trying to receive from the {source}: {source} event \
				 channel disconnected"
			)),
		}
	}

	/// Receives and handles all incoming events from the [event
	/// channel](Self::event_channel).
	fn process_all_events(&mut self, tui: &mut Tui) -> crate::Result<()> {
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
			AppEvent::Tick => self.tick()?,
			AppEvent::Render => self.render(tui)?,
			AppEvent::Close => {
				self.close();
			},
			AppEvent::Quit => {
				self.quit();
			},
			AppEvent::ErrorOccurred(msg) => self.error_occurred(msg)?,
			AppEvent::UserInput(input) => self.user_input(tui, input)?,
			AppEvent::ManipulateScreen(_) => {},
		}
		Ok(())
	}

	/// Updates the state of the app.
	fn tick(&mut self) -> crate::Result<()> {
		Ok(())
	}

	/// Renders the active screen.
	fn render(&mut self, tui: &mut Tui) -> crate::Result<()> {
		self.screen_handler.render(&mut tui.get_frame())
	}

	/// Sets the app's state to closing.
	fn close(&mut self) {
		self.set_run_state(AppRunState::Closing);
		self.screen_handler.close();
	}

	/// Quits the app.
	fn quit(&mut self) {
		self.set_run_state(AppRunState::Quitting);
		self.screen_handler.quit();
	}

	/// Handles a [`UserInput`](InputEvent).
	fn user_input(
		&mut self,
		tui: &mut Tui,
		input: &InputEvent,
	) -> crate::Result<()> {
		if let InputEvent::ResizeTerminal(w, h) = input {
			self.resize_terminal(tui, (*w, *h))
		} else {
			Ok(())
		}
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
		(w, h): (u16, u16),
	) -> crate::Result<()> {
		tui.resize(Rect::new(0, 0, w, h))?;
		self.event_channel.send(Event::App(AppEvent::Render))?;
		Ok(())
	}
}
