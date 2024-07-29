//! Interface bridge between the application and the terminal. Uses
//! [`crossterm`] and [`ratatui`] internally.

use std::{
	io::{
		stdout,
		Stdout,
	},
	ops::{
		Deref,
		DerefMut,
	},
	time::Duration,
};

use color_eyre::eyre::eyre;
use crossterm::{
	cursor::{
		DisableBlinking,
		EnableBlinking,
		Hide,
		MoveTo,
		Show,
	},
	event::{
		DisableBracketedPaste,
		DisableFocusChange,
		DisableMouseCapture,
		EnableBracketedPaste,
		EnableFocusChange,
		EnableMouseCapture,
		Event as CrosstermEvent,
		EventStream as CrosstermEventStream,
		KeyEvent,
		MouseEvent,
	},
	execute,
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
		EnterAlternateScreen,
		LeaveAlternateScreen,
	},
};
use derive_new::new;
use futures::{
	FutureExt,
	StreamExt,
};
use ratatui::prelude::CrosstermBackend;
use tokio::{
	sync::mpsc::UnboundedSender,
	task::JoinHandle,
	time::interval,
};
use tokio_util::sync::CancellationToken;
use tracing::{
	debug,
	error,
	info,
	instrument,
	trace,
	warn,
};

use crate::{
	config::GameSpecs,
	util::UnboundedChannel,
};

/// Terminal type used by Terminal Arcade.
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

/// Handler for processing terminal-related events and producing application
/// events. This struct also has [`Deref`] and [`DerefMut`] implementations to
/// the contained [`Tui::terminal`]. When this struct is [`Drop`]ped,
/// [`Tui::exit`] will be called.
///
/// Note that by default, mouse capture is not enabled.
///
/// This struct provides several methods to influence its control flow:
/// * [`Tui::start`] starts terminal event handling.
/// * [`Tui::stop`] stops terminal event handling.
/// * [`Tui::enter`] enters the terminal interface itself. **This should be used
///   most if not all of the time.**
/// * [`Tui::exit`] exits the terminal interface.
/// * [`Tui::suspend`] *suspends* Terminal Arcade.
///
/// To begin, see [`Tui::enter`] and [`Tui::exit`] for the recommended ways
/// to start the terminal interface. Typically, only [`Tui::enter`] will need
/// to be called after [creating](Tui::new) a TUI object, as dropping it will
/// automatically make it [exit](Tui::exit).
#[derive(Debug, new)]
pub struct Tui {
	/// Terminal interface to interact with.
	terminal: Terminal,

	/// Handle for event task.
	event_task: JoinHandle<()>,

	/// This handler's cancellation token.
	cancel_token: CancellationToken,

	/// [`TuiEvent`] channel.
	event_channel: UnboundedChannel<TuiEvent>,

	/// Tick rate - how rapidly to update state.
	tick_rate: Duration,

	/// Frame rate - how rapidly to render.
	frame_rate: Duration,
}

impl Tui {
	/// Constructs a new terminal interface object with the provided
	/// [`GameSpecs`].
	pub fn with_specs(specs: GameSpecs) -> crate::Result<Self> {
		Ok(Self {
			terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
			event_task: tokio::spawn(async {}),
			cancel_token: CancellationToken::new(),
			event_channel: UnboundedChannel::new(),
			tick_rate: Duration::try_from_secs_f64(1.0 / specs.tps)?,
			frame_rate: Duration::try_from_secs_f64(1.0 / specs.fps)?,
		})
	}

	/// Receives the next [TUI event](TuiEvent).
	pub async fn next_event(&mut self) -> Option<TuiEvent> {
		self.event_channel.get_mut_receiver().recv().await
	}

	/// Event loop to interact with the terminal.
	#[instrument(
		level = "info",
		name = "terminal-event-loop",
		skip(event_sender, cancel_token)
	)]
	#[allow(clippy::ignored_unit_patterns)]
	async fn event_loop(
		event_sender: UnboundedSender<TuiEvent>,
		cancel_token: CancellationToken,
		tick_rate: Duration,
		frame_rate: Duration,
	) {
		let mut event_stream = CrosstermEventStream::new();
		let mut tick_interval = interval(tick_rate);
		let mut render_interval = interval(frame_rate);

		if let Err(err) = event_sender.send(TuiEvent::Init) {
			error!(%err, "unable to send initial event");
			return;
		}

		loop {
			let tui_event = tokio::select! {
				_ = cancel_token.cancelled() => {
					debug!("tui's cancel token cancelled");
					break;
				}
				_ = tick_interval.tick() => TuiEvent::Tick,
				_ = render_interval.tick() => TuiEvent::Render,
				crossterm_event = event_stream.next().fuse() => match crossterm_event {
					Some(Ok(event)) => {
						event.into()
					},
					Some(Err(err)) => {
						error!(%err, "while receiving from event stream");
						TuiEvent::Error(err.to_string())
					},
					None => {
						warn!("event stream closed; no more events are to be consumed");
						break;
					}
				},
			};
			if let Err(err) = event_sender.send(tui_event) {
				error!("failed to send tui event: {err}; quitting now");
				break;
			}
		}
		info!("tui event loop is finished");
	}

	/// Begins event reception and enters the terminal.
	#[instrument(skip(self))]
	pub fn enter(&mut self) -> crate::Result<()> {
		info!("entering the tui");
		self.clear()?;
		Self::set_terminal_rules()?;
		self.start();
		Ok(())
	}

	/// Exits the terminal interface.
	#[instrument(skip(self))]
	pub fn exit(&mut self) -> crate::Result<()> {
		info!("exiting the tui");
		self.clear()?;
		self.stop()?;
		Self::reset_terminal_rules()?;
		Ok(())
	}

	/// (Re-)starts the terminal interface layer.
	#[instrument(skip(self))]
	pub fn start(&mut self) {
		self.cancel_token.cancel(); // To cancel any existing tasks.
		self.cancel_token = CancellationToken::new();

		let event_loop = Self::event_loop(
			self.event_channel.get_sender().clone(),
			self.cancel_token.clone(),
			self.tick_rate,
			self.frame_rate,
		);
		self.event_task = tokio::spawn(event_loop);
	}

	/// Stops the terminal interface layer. After 100ms, forcefully aborts
	/// [`Self::event_task`] and returns if that is unsuccessful after 200ms.
	#[instrument(skip(self))]
	pub fn stop(&mut self) -> crate::Result<()> {
		self.cancel_token.cancel();
		let one_ms = Duration::from_millis(1);
		let mut cancel_timer = Duration::from_millis(0);
		let mut aborting = false;

		while !self.event_task.is_finished() {
			std::thread::sleep(one_ms);
			cancel_timer += one_ms;

			if cancel_timer > Duration::from_millis(100) && !aborting {
				warn!(
					"could not cancel event task thread after 100ms; aborting \
					 it"
				);
				aborting = true;
				self.event_task.abort();
			} else if cancel_timer > Duration::from_millis(200) {
				let message = "could not abort event task thread after 200ms";
				error!("{message}; exiting");
				return Err(eyre!(message));
			}
		}
		Ok(())
	}

	/// Sets global terminal rules.
	pub fn set_terminal_rules() -> crate::Result<()> {
		enable_raw_mode()?;
		execute!(
			stdout(),
			EnableBracketedPaste,
			EnableFocusChange,
			DisableBlinking,
			EnterAlternateScreen,
			Hide,
			MoveTo(0, 0)
		)?;
		Ok(())
	}

	/// Resets global terminal rules set by [`Self::set_terminal_rules`].
	pub fn reset_terminal_rules() -> crate::Result<()> {
		disable_raw_mode()?;
		execute!(
			stdout(),
			DisableBracketedPaste,
			DisableMouseCapture,
			DisableFocusChange,
			EnableBlinking,
			LeaveAlternateScreen,
			Show,
		)?;
		Ok(())
	}

	/// Enables mouse capture.
	pub fn enable_mouse_capture() -> crate::Result<()> {
		execute!(stdout(), EnableMouseCapture)?;
		Ok(())
	}

	/// Disables mouse capture.
	pub fn disable_mouse_capture() -> crate::Result<()> {
		execute!(stdout(), DisableMouseCapture)?;
		Ok(())
	}
}

impl Deref for Tui {
	type Target = Terminal;

	fn deref(&self) -> &Self::Target {
		&self.terminal
	}
}

impl DerefMut for Tui {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.terminal
	}
}

impl Drop for Tui {
	fn drop(&mut self) {
		if let Err(err) = self.exit() {
			panic!("could not exit the tui (when dropping): {err}");
		}
	}
}

/// Events sent by [`Tui`].
#[derive(Debug, Clone, Hash)]
pub enum TuiEvent {
	/// Checks if event transmission works.
	Init,

	/// Updates game state.
	Tick,

	/// Renders the application to the terminal.
	Render,

	/// An error occurred (while retrieving the next terminal event from
	/// [`EventStream`](CrosstermEventStream)).
	Error(String),

	/// A key is inputted by the user.
	Key(KeyEvent),

	/// The mouse is manipulated by the user.
	Mouse(MouseEvent),

	/// Some text is pasted by the user.
	Paste(String),

	/// The terminal is resized to `(width, height)`.
	Resize(u16, u16),

	/// The terminal changed focus.
	Focus(FocusChange),
}

/// A change in focus of the terminal.
#[derive(Debug, Clone, Copy, Hash)]
#[allow(missing_docs)] // Obvious variant names
pub enum FocusChange {
	Lost,
	Gained,
}

impl From<CrosstermEvent> for TuiEvent {
	fn from(value: CrosstermEvent) -> Self {
		match value {
			CrosstermEvent::Key(key) => Self::Key(key),
			CrosstermEvent::Mouse(mouse) => Self::Mouse(mouse),
			CrosstermEvent::Paste(text) => Self::Paste(text),
			CrosstermEvent::Resize(width, height) => {
				Self::Resize(width, height)
			},
			CrosstermEvent::FocusLost => Self::Focus(FocusChange::Lost),
			CrosstermEvent::FocusGained => Self::Focus(FocusChange::Gained),
		}
	}
}
