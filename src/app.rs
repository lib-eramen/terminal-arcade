//! Handles the state of the application, including the actual data as well as
//! rendering of it.
//!
//! Note that the terms "close" and "quit" aren't synonyms the way they are used
//! here - "quitting" implies that the exits immediately (in other words,
//! force-quit), while "closing" doesn't.

use std::{
	cell::RefCell,
	rc::Rc,
};

use color_eyre::eyre::eyre;
use derive_new::new;
use tokio::sync::mpsc::error::TryRecvError;
use tracing::instrument;

use crate::{
	components::screens::home::HomeScreen,
	config::Config,
	events::{
		AppEvent,
		Event,
		TuiAppMiddleman,
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
#[derive(Debug, new)]
pub struct App {
	/// Running state of the app.
	run_state: AppRunState,

	/// Tui backing the app.
	tui: Tui,

	/// Middleman processing events between the [`Tui`] and [itself](App).
	middleman: TuiAppMiddleman,

	/// UI of the app.
	ui: Ui,

	/// App config.
	config: Rc<RefCell<Config>>,

	/// [`Event`] channel. The sender of this channel is cloned for screens to
	/// send their own events to the app.
	event_channel: UnboundedChannel<Event>,
}

impl App {
	/// Constructs a new app witht the provided [`Config`].
	pub fn with_config(config: Config) -> crate::Result<Self> {
		let tui = Tui::with_specs(&config.game_specs)?;
		let terminal = tui.terminal.clone();
		let event_channel = UnboundedChannel::new();
		let event_sender = event_channel.get_sender().clone();

		Ok(Self {
			run_state: AppRunState::Pending,
			tui,
			middleman: TuiAppMiddleman::new(event_sender.clone()),
			ui: Ui::new(terminal, event_sender),
			config: Rc::new(RefCell::new(config)),
			event_channel,
		})
	}

	/// Starts the app with the provided terminal interface and with a landing
	/// [`HomeScreen`].
	#[instrument(name = "run-app", skip_all)]
	pub fn run(&mut self) -> crate::Result<()> {
		tracing::debug!(?self.config, "using provided config");
		self.set_run_state(AppRunState::Running);
		self.tui.enter()?;
		self.ui.push_active_screen(HomeScreen)?;
		self.event_loop()?;
		println!("See you next time! 🕹️ 👋");
		Ok(())
	}

	/// App event loop.
	fn event_loop(&mut self) -> crate::Result<()> {
		loop {
			self.relay_tui_event()?;
			self.process_all_events()?;
			self.update()?;
			if self.run_state == AppRunState::Finished {
				break;
			}
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

	/// Handles an event received from from the provided [`Tui`], transforms the
	/// event with the [middleman](TuiAppMiddleman), then sends the resulting
	/// [`AppEvent`] through the [channel], if there is any.
	fn relay_tui_event(&mut self) -> crate::Result<()> {
		let tui_event = match self.tui.try_recv_event() {
			Ok(event) => event,
			Err(err) => return Self::handle_try_recv_err(err, "tui"),
		};
		self.middleman.handle_tui_event(tui_event)?;
		Ok(())
	}

	/// Receives and handles all incoming events from the [event
	/// channel](Self::event_channel).
	fn process_all_events(&mut self) -> crate::Result<()> {
		loop {
			match self.event_channel.try_recv() {
				Ok(event) => self.event(event)?,
				Err(err) => return Self::handle_try_recv_err(err, "app"),
			}
		}
	}

	/// Returns whether the app has reached a point where it can be declared
	/// [finished](AppRunState::Finished).
	fn can_be_finished(&self) -> bool {
		self.run_state == AppRunState::Quitting
			|| self.ui.get_run_state() == UiRunState::Finished
	}

	/// Sets the run state of the app.
	fn set_run_state(&mut self, run_state: AppRunState) {
		tracing::info!(?run_state, "setting app run state");
		self.run_state = run_state;
	}

	/// Handles an [`AppEvent`].
	fn handle_app_event(&mut self, event: &AppEvent) -> crate::Result<()> {
		match event {
			AppEvent::Render => self.render()?,
			AppEvent::Close => self.close(),
			AppEvent::Quit => self.quit(),
			AppEvent::Error(msg) => self.error(msg)?,
			AppEvent::Tick(_) => {},
		}
		Ok(())
	}

	/// Updates the app.
	fn update(&mut self) -> crate::Result<()> {
		self.ui.update()?;
		if self.can_be_finished() {
			self.set_run_state(AppRunState::Finished);
		}
		Ok(())
	}

	/// Handles a given event.
	fn event(&mut self, event: Event) -> crate::Result<()> {
		if event.should_be_logged() {
			tracing::info!(?event, "receiving event");
		}
		if let Event::App(ref app_event) = event {
			self.handle_app_event(app_event)?;
		}
		self.ui.event(event)
	}

	/// Renders the app.
	fn render(&mut self) -> crate::Result<()> {
		self.ui
			.render(&mut self.tui.terminal.borrow_mut().get_frame())
	}

	/// Sets the app's state to closing.
	fn close(&mut self) {
		self.set_run_state(AppRunState::Closing);
		self.ui.close();
	}

	/// Quits the app.
	fn quit(&mut self) {
		self.set_run_state(AppRunState::Quitting);
		self.ui.quit();
	}

	/// Logs the error and displays it on a popup in the terminal.
	fn error(&mut self, msg: &str) -> crate::Result<()> {
		tracing::error!(msg, "an error event occurred");
		todo!();
	}
}
