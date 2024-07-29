//! Handles the state of the application, including the actual data as well as
//! rendering of it.

use color_eyre::eyre::eyre;
use crossterm::event::{
	KeyEvent,
	MouseEvent,
};
use derive_new::new;
use ratatui::layout::Rect;
use serde::{
	Deserialize,
	Serialize,
};
use tracing::{
	debug,
	error,
	info,
	instrument,
	trace,
};

use crate::{
	config::Config,
	event::Event,
	tui::{
		FocusChange,
		Tui,
		TuiEvent,
	},
	ui::screen::handler::ScreenHandler,
	util::UnboundedChannel,
};

/// Running state of the application.
#[derive(
	Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
#[allow(missing_docs)] // Relatively obvious variant names
pub enum RunState {
	/// The app has not started.
	#[default]
	Pending,

	/// The app is running.
	Running,

	/// The app is exiting. The included boolean indicates
	/// whether or not this action is forced.
	Quitting(bool),
}

/// Handler for application state and rendering.
#[derive(Debug, Serialize, Deserialize, new)]
pub struct App {
	/// Running state of the application.
	run_state: RunState,

	/// Application configuration.
	config: Config,

	/// Screen handler.
	screen_handler: ScreenHandler,

	/// [`Event`] channel.
	event_channel: UnboundedChannel<Event>,

	/// Key events that accumulate in one tick.
	#[serde(skip)]
	tick_key_events: Vec<KeyEvent>,
}

impl App {
	/// Constructs a new Terminal Arcade app with the provided configuration.
	pub fn with_config(config: Config) -> Self {
		debug!(?config, "using provided config");
		Self::new(
			RunState::default(),
			config,
			ScreenHandler::default(),
			UnboundedChannel::default(),
			Vec::default(),
		)
	}

	/// Starts the app with the provided terminal interface.
	#[instrument(name = "run-app", skip(self, tui))]
	pub async fn run(mut self, mut tui: Tui) -> crate::Result<()> {
		self.run_state = RunState::Running;
		tui.enter()?;
		self.event_loop(tui).await?;
		Ok(())
	}

	/// Quits the app.
	#[instrument(name = "quit-app", skip(self, _tui))]
	pub async fn quit(&mut self, _tui: Tui, forced: bool) -> crate::Result<()> {
		Ok(())
	}

	/// App event loop.
	async fn event_loop(&mut self, mut tui: Tui) -> crate::Result<()> {
		loop {
			self.handle_tui_event(&mut tui).await?;
			self.handle_app_events(&mut tui).await?;
			if let RunState::Quitting(forced) = self.run_state {
				info!("quitting the app");
				self.quit(tui, forced).await?;
				break;
			}
		}
		Ok(())
	}

	/// Renders the application to the terminal.
	fn render(&mut self, tui: &mut Tui) -> crate::Result<()> {
		tui.draw(|frame| todo!())?;
		Ok(())
	}

	/// Handles a terminal resizing event.
	fn resize(
		&mut self,
		tui: &mut Tui,
		width: u16,
		height: u16,
	) -> crate::Result<()> {
		tui.resize(Rect::new(0, 0, width, height))?;
		self.event_channel
			.get_sender()
			.send(Event::App(AppEvent::Render))?;
		Ok(())
	}

	/// Handles an event received from from the provided [`Tui`], then.
	async fn handle_tui_event(&mut self, tui: &mut Tui) -> crate::Result<()> {
		let Some(event) = tui.next_event().await else {
			info!("no tui events were received");
			return Ok(());
		};
		trace!(?event, "received tui event");

		let event_sender = self.event_channel.get_sender().clone();
		match event {
			TuiEvent::Init => {
				info!("thanks for the init event! very considerate");
			},
			TuiEvent::Tick => event_sender.send(Event::App(AppEvent::Tick))?,
			TuiEvent::Render => {
				event_sender.send(Event::App(AppEvent::Render))?;
			},
			TuiEvent::Error(msg) => {
				event_sender.send(Event::App(AppEvent::Error(format!(
					"something happened on the tui side: {msg}"
				))))?;
			},
			TuiEvent::Key(key) => self.tick_key_events.push(key),
			TuiEvent::Mouse(mouse) => {
				event_sender.send(Event::App(AppEvent::Mouse(mouse)))?;
			},
			TuiEvent::Paste(text) => {
				event_sender.send(Event::App(AppEvent::Paste(text)))?;
			},
			TuiEvent::Resize(width, height) => {
				self.resize(tui, width, height)?;
			},
			TuiEvent::Focus(change) => {
				event_sender.send(Event::App(AppEvent::Focus(change)))?;
			},
		}
		Ok(())
	}

	/// Handles an [app event](AppEvent).
	async fn handle_app_event(
		&mut self,
		tui: &mut Tui,
		event: AppEvent,
	) -> crate::Result<()> {
		if event.should_be_logged() {
			debug!(app_event = ?event, "received app event"); // TODO: Mouse events?
		}
		let event_sender = self.event_channel.get_sender();
		match event {
			AppEvent::Tick => {
				event_sender.send(Event::App(AppEvent::Keys(
					self.tick_key_events.drain(..).collect(),
				)))?;
			},
			AppEvent::Render => self.render(tui)?,
			AppEvent::Quit(forced) => {
				self.indicate_quit(forced)?;
			},
			AppEvent::Error(msg) => {
				let msg = format!("received an error message: {msg}");
				error!(msg, "received an error");
				// TODO: Additional error handling (i.e. displaying it on a
				// popup), putting the screen that caused the error in the
				// log there
			},
			AppEvent::Keys(keys) => todo!(),
			AppEvent::Mouse(mouse) => todo!(),
			AppEvent::Paste(text) => todo!(),
			AppEvent::Resize(width, height) => todo!(),
			AppEvent::Focus(change) => {},
		}
		Ok(())
	}

	/// Receives and handles all incoming events from the [event
	/// channel](Self::event_channel).
	async fn handle_app_events(&mut self, tui: &mut Tui) -> crate::Result<()> {
		while let Ok(Event::App(event)) =
			self.event_channel.get_mut_receiver().try_recv()
		{
			self.handle_app_event(tui, event).await?;
		}
		Ok(())
	}

	/// Indicats that the app should be quitting. Rather than calling
	/// [`Self::quit`] directly, this is the preferred way to end the app by
	/// setting an internal [flag](Self::run_state) to indicate the run state.
	///
	/// This method will return an error if the state is already quitting.
	fn indicate_quit(&mut self, forced: bool) -> crate::Result<()> {
		if let RunState::Quitting(_) = self.run_state {
			Err(eyre!("app is already set to quitting"))
		} else {
			self.run_state = RunState::Quitting(forced);
			info!("set app's run state; indicated to quit");
			Ok(())
		}
	}
}

/// Events sent by [`Tui`].
#[derive(Debug, Clone, Hash)]
pub enum AppEvent {
	/// Updates game state.
	Tick,

	/// Renders the application to the terminal.
	Render,

	/// Quits the application. The included boolean indicates whether
	/// the action is forced
	Quit(bool),

	/// An error occurred in the application, sent with the provided message.
	Error(String),

	/// Key events accumulated from one [tick](Self::Tick).
	Keys(Vec<KeyEvent>),

	/// The mouse is manipulated by the user.
	Mouse(MouseEvent),

	/// Some text is pasted by the user.
	Paste(String),

	/// The terminal is resized to `(width, height)`.
	Resize(u16, u16),

	/// The terminal changed focus.
	Focus(FocusChange),
}

impl AppEvent {
	/// Returns whether this event should be logged.
	pub fn should_be_logged(&self) -> bool {
		!matches!(self, Self::Tick | Self::Render)
	}
}
