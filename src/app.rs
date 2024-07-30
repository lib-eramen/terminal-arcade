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
			self.handle_app_events(&mut tui)?;
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
		tui.clear()?;
		tui.draw(|frame| {})?;
		Ok(())
	}

	/// Resizes the terminal to the provided dimensions.
	fn resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> crate::Result<()> {
		tui.resize(Rect::new(0, 0, w, h))?;
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
		if let TuiEvent::Init = event {
			info!("received init event! very considerate of you, kind tui");
			Ok(())
		} else {
			self.send_app_event(event.into())
		}
	}

	/// Handles an [app event](AppEvent).
	fn handle_app_event(
		&mut self,
		tui: &mut Tui,
		event: AppEvent,
	) -> crate::Result<()> {
		match event {
			// TODO: Buffer for state-affecting events, not just keys (maybe
			// with a type). Documentation in some of these methods will also
			// have to change.
			AppEvent::Tick => self.handle_tick_event()?,
			AppEvent::Render => self.render(tui)?,
			AppEvent::Quit(forced) => self.indicate_quit(forced)?,
			AppEvent::Error(ref msg) => self.handle_error_event(msg)?,
			AppEvent::Key(key) => self.tick_key_events.push(key),
			AppEvent::Keys(keys) => self.handle_keys_event(keys)?,
			AppEvent::Mouse(mouse) => self.handle_mouse_event(mouse)?,
			AppEvent::Paste(text) => self.handle_paste_event(text)?,
			AppEvent::Resize(w, h) => self.resize(tui, w, h)?,
			AppEvent::Focus(change) => self.handle_focus_event(change)?,
		}
		Ok(())
	}

	/// Receives and handles all incoming events from the [event
	/// channel](Self::event_channel).
	fn handle_app_events(&mut self, tui: &mut Tui) -> crate::Result<()> {
		while let Ok(Event::App(event)) =
			self.event_channel.get_mut_receiver().try_recv()
		{
			self.handle_app_event(tui, event)?;
		}
		Ok(())
	}

	/// Handles an [`AppEvent::Quit`] event by [indicating a quit
	/// status](Self::indicate_quit).
	fn handle_quit_event(&mut self, forced: bool) -> crate::Result<()> {
		self.indicate_quit(forced)?;
		Ok(())
	}

	/// Handles an [`AppEvent::Tick`] event by sending a [`Keys`]
	/// (`AppEvent::Keys`) event with the [`KeyEvent`]s [`drain`](Vec::drain)ed
	/// from [`Self::tick_key_events`].
	fn handle_tick_event(&mut self) -> crate::Result<()> {
		if !self.tick_key_events.is_empty() {
			let key_events = self.tick_key_events.drain(..).collect();
			self.send_app_event(AppEvent::Keys(key_events))?;
		}
		Ok(())
	}

	/// Handles an [`AppEvent::Error`] event.
	fn handle_error_event(&mut self, msg: &str) -> crate::Result<()> {
		let msg = format!("received an error message: {msg}");
		error!(msg, "received an error");
		// TODO: Additional error handling (i.e. displaying it on a
		// popup), putting the screen that caused the error in the
		// log there
		Ok(())
	}

	/// Handles an [`AppEvent::Keys`] event with the provided buffer of key
	/// events, preferably gotten from draining [`Self::tick_key_events`].
	fn handle_keys_event(&mut self, keys: Vec<KeyEvent>) -> crate::Result<()> {
		// TODO: Handle keys event - placeholder is quitting on any key input.
		info!("placeholder key response - quitting");
		self.send_app_event(AppEvent::Quit(false))?;
		Ok(())
	}

	/// Handles an [`AppEvent::Mouse`] event.
	fn handle_mouse_event(&mut self, mouse: MouseEvent) -> crate::Result<()> {
		// TODO: Handle mouse event
		Ok(())
	}

	/// Handles an [`AppEvent::Paste`] event.
	fn handle_paste_event(&mut self, text: String) -> crate::Result<()> {
		// TODO: Handle paste
		Ok(())
	}

	/// Handles an [`AppEvent::Focus`] event.
	fn handle_focus_event(&mut self, change: FocusChange) -> crate::Result<()> {
		// TODO: Handle focus change
		Ok(())
	}

	/// Sends an [`AppEvent`] through this struct's
	/// [channel](Self::event_channel).
	fn send_app_event(&self, app_event: AppEvent) -> crate::Result<()> {
		if app_event.should_be_logged() {
			debug!(?app_event, "sending app event");
		}
		self.event_channel
			.get_sender()
			.send(Event::App(app_event))?;
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

	/// A key is inputted by the user.
	Key(KeyEvent),

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
	/// Returns whether this event should be logged. This function will return
	/// `false` for repetitive events ([`Self::Tick`] and [`Self::Render`]) and
	/// for individual events that should be buffered and released with every
	/// app tick.
	pub fn should_be_logged(&self) -> bool {
		!matches!(self, Self::Tick | Self::Render | Self::Key(_))
	}
}

impl From<TuiEvent> for AppEvent {
	/// Converts a [`TuiEvent`] to an [`AppEvent`]. Panics if the [`TuiEvent`]
	/// is a [`TuiEvent::Init`] event.
	fn from(value: TuiEvent) -> Self {
		match value {
			TuiEvent::Init => {
				panic!("cannot convert init tui event to app event")
			},
			TuiEvent::Tick => Self::Tick,
			TuiEvent::Render => Self::Render,
			TuiEvent::Error(msg) => Self::Error(msg),
			TuiEvent::Key(key) => Self::Key(key),
			TuiEvent::Mouse(mouse) => Self::Mouse(mouse),
			TuiEvent::Paste(text) => Self::Paste(text),
			TuiEvent::Resize(w, h) => Self::Resize(w, h),
			TuiEvent::Focus(change) => Self::Focus(change),
		}
	}
}
