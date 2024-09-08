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

use color_eyre::{
	eyre::eyre,
	Section,
};
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
		EventStream as CrosstermEventStream,
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
use serde::{
	Deserialize,
	Serialize,
};
use tokio::{
	sync::mpsc::{
		error::TryRecvError,
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

use crate::{
	events::TuiEvent,
	utils::UnboundedChannel,
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
/// This struct provides two methods to influence its control flow:
/// [`Tui::start`] and [`Tui::stop`] (which gets called when dropping this
/// struct).
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
	event_task: JoinHandle<crate::Result<()>>,

	/// This handler's cancellation token.
	pub cancel_token: CancellationToken,

	/// [`TuiEvent`] channel.
	pub event_channel: UnboundedChannel<TuiEvent>,

	/// Tick rate - how rapidly to update state.
	tick_rate: Duration,

	/// Frame rate - how rapidly to render.
	frame_rate: Duration,
}

impl Tui {
	/// Constructs a new terminal interface object with the provided
	/// [`GameSpecs`].
	pub fn with_specs(game_specs: &GameSpecs) -> crate::Result<Self> {
		Ok(Self::new(
			Terminal::new(CrosstermBackend::new(stdout()))?,
			tokio::spawn(async move { Ok(()) }),
			CancellationToken::new(),
			UnboundedChannel::new(),
			game_specs.get_tick_rate()?,
			game_specs.get_frame_rate()?,
		))
	}

	/// Tries to receive the next [TUI event](TuiEvent).
	pub fn try_recv_event(&mut self) -> Result<TuiEvent, TryRecvError> {
		self.event_channel.try_recv()
	}

	/// Sends one [TUI event](TuiEvent) to the [event
	/// channel](Self::event_channel).
	fn send_tui_event(
		event_sender: &UnboundedSender<TuiEvent>,
		tui_event: TuiEvent,
	) -> crate::Result<()> {
		if tui_event.should_be_logged() {
			debug!(?tui_event, "sending tui event");
		}
		event_sender.send(tui_event)?;
		Ok(())
	}

	/// Event loop to interact with the terminal.
	#[instrument(level = "info", name = "terminal-event-loop", skip_all)]
	async fn event_loop(
		event_sender: UnboundedSender<TuiEvent>,
		cancel_token: CancellationToken,
		tick_rate: Duration,
		frame_rate: Duration,
	) -> crate::Result<()> {
		let mut event_stream = CrosstermEventStream::new();
		let mut tick_interval = interval(tick_rate);
		let mut render_interval = interval(frame_rate);

		if let Err(err) = event_sender.send(TuiEvent::Hello) {
			return Err(eyre!("while sending greetings! how rude: {err}"));
		}

		loop {
			let tui_event = tokio::select! {
				() = cancel_token.cancelled() => {
					info!("tui's cancel token cancelled");
					break;
				},
				() = event_sender.closed() => {
					info!("event sender closed");
					break;
				},
				_ = tick_interval.tick() => TuiEvent::Tick,
				_ = render_interval.tick() => TuiEvent::Render,
				crossterm_event = event_stream.next().fuse() => match crossterm_event {
					Some(Ok(event)) => {
						event.into()
					},
					Some(Err(err)) => {
						return Err(eyre!("while receiving from event stream: {err}"));
					},
					None => {
						warn!("event stream closed; no more events are to be consumed");
						break;
					}
				},
			};
			if let Err(err) =
				Self::send_tui_event(&event_sender, tui_event.clone())
			{
				return Err(eyre!("while sending tui event: {err}").with_note(
					|| format!("trying to send event: {tui_event:?}"),
				));
			}
		}
		info!("tui event loop is finished");
		Ok(())
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
			Hide,
			EnableBracketedPaste,
			EnableFocusChange,
			DisableBlinking,
			EnterAlternateScreen,
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

/// Wrapper struct around two [`f64`]s for the ticks per second and the frames
/// per second numbers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, new)]
pub struct GameSpecs {
	/// Ticks per second.
	pub tps: f64,

	/// Frames per second.
	pub fps: f64,
}

impl GameSpecs {
	pub fn get_tick_rate(&self) -> crate::Result<Duration> {
		Ok(Duration::try_from_secs_f64(1.0 / self.tps)?)
	}

	pub fn get_frame_rate(&self) -> crate::Result<Duration> {
		Ok(Duration::try_from_secs_f64(1.0 / self.fps)?)
	}
}

impl Default for GameSpecs {
	fn default() -> Self {
		Self::new(16.0, 60.0)
	}
}
