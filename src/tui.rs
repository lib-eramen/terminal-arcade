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
use futures::{
	FutureExt,
	StreamExt,
};
use ratatui::prelude::CrosstermBackend;
use tokio::{
	sync::mpsc::{
		UnboundedReceiver,
		UnboundedSender,
	},
	task::JoinHandle,
	time::interval,
};
use tokio_util::sync::CancellationToken;
use tracing::{
	debug,
	error,
	info,
	instrument,
	warn,
};

/// Terminal type used by Terminal Arcade.
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

/// Handler for processing terminal-related events and producing application
/// events. This struct also has [`Deref`] and [`DerefMut`] implementations to
/// the contained [`Tui::terminal`]. When this struct is [`Drop`]ped,
/// [`Tui::exit`] will be called.
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
#[derive(Debug)]
pub struct Tui {
	/// Terminal interface to interact with.
	terminal: Terminal,

	/// Handle for event task.
	event_task: JoinHandle<()>,

	/// This handler's cancellation token.
	cancel_token: CancellationToken,

	/// Event sender channel.
	event_sender: UnboundedSender<TuiEvent>,

	/// Event receiver channel.
	event_receiver: UnboundedReceiver<TuiEvent>,

	/// Tick rate - how rapidly to update state.
	tick_rate: Duration,

	/// Frame rate - how rapidly to render.
	frame_rate: Duration,
}

impl Tui {
	/// Constructs a new terminal interface object.
	pub fn new(tps: u32, fps: u32) -> crate::Result<Self> {
		let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
		Ok(Self {
			terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
			event_task: tokio::spawn(async {}),
			cancel_token: CancellationToken::new(),
			event_sender: sender,
			event_receiver: receiver,
			tick_rate: Duration::from_secs_f64(1.0 / f64::from(tps)),
			frame_rate: Duration::from_secs_f64(1.0 / f64::from(fps)),
		})
	}

	/// Receives the next [TUI event](TuiEvent).
	pub async fn next_event(&mut self) -> Option<TuiEvent> {
		self.event_receiver.recv().await
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
	) -> () {
		let mut event_stream = CrosstermEventStream::new();
		let mut tick_interval = interval(tick_rate);
		let mut render_interval = interval(frame_rate);

		if let Err(err) = event_sender.send(TuiEvent::Init) {
			error!("unable to send initial event: {err}");
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
						error!("while receiving from event stream: {err}");
						TuiEvent::Error(err.to_string())
					},
					None => {
						warn!("event stream closed; no more events are to be consumed");
						break;
					}
				},
			};
			debug!("sending crossterm event: {tui_event:?}");
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
		Self::set_terminal_rules()?;
		self.start();
		Ok(())
	}

	/// Exits the terminal interface.
	#[instrument(skip(self))]
	pub fn exit(&mut self) -> crate::Result<()> {
		info!("exiting the tui");
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
			self.event_sender.clone(),
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

	/// Suspends the terminal interface - think force-quit-ish [`Self::exit`].
	/// Also [raise](signal_hook::low_level::raise)s a
	/// [`SIGTSTP`](libc::SIGTSTP) signal.
	#[instrument(skip(self))]
	pub fn suspend(&mut self) -> crate::Result<()> {
		self.exit()?;
		#[cfg(not(windows))]
		signal_hook::low_level::raise(libc::SIGTSTP)?;
		Ok(())
	}

	/// Resumes the terminal interface. Analogous to calling [`Self::enter`]
	/// directly.
	#[instrument(skip(self))]
	pub fn resume(&mut self) -> crate::Result<()> {
		self.enter()?;
		Ok(())
	}

	/// Sets global terminal rules.
	fn set_terminal_rules() -> crate::Result<()> {
		enable_raw_mode()?;
		execute!(
			stdout(),
			EnableBracketedPaste,
			EnableMouseCapture,
			EnableFocusChange,
			DisableBlinking,
			EnterAlternateScreen,
			Hide,
			MoveTo(0, 0)
		)?;
		Ok(())
	}

	/// Resets global terminal rules set by [`Self::set_terminal_rules`].
	pub(crate) fn reset_terminal_rules() -> crate::Result<()> {
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

	/// The terminal is resized to `(x, y)`.
	Resize(u16, u16),

	/// The terminal lost focus.
	FocusLost,

	/// The terminal gained focus.
	FocusGained,
}

impl From<CrosstermEvent> for TuiEvent {
	fn from(value: CrosstermEvent) -> Self {
		match value {
			CrosstermEvent::Key(key) => Self::Key(key),
			CrosstermEvent::Mouse(mouse) => Self::Mouse(mouse),
			CrosstermEvent::Paste(text) => Self::Paste(text),
			CrosstermEvent::Resize(x, y) => Self::Resize(x, y),
			CrosstermEvent::FocusLost => Self::FocusLost,
			CrosstermEvent::FocusGained => Self::FocusGained,
		}
	}
}
