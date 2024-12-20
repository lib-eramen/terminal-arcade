//! Interface bridge between the application and the terminal. Uses
//! [`crossterm`] and [`ratatui`] internally.

use std::{
	io::{
		stdout,
		Stdout,
	},
	time::{
		Duration,
		TryFromFloatSecsError,
	},
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
		DisableFocusChange,
		DisableMouseCapture,
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

use crate::{
	events::StationEvent,
	utils::UnboundedChannel,
};

/// App terminal type.
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

/// Game specifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSpecs {
	/// Ticks per second.
	pub tps: f64,

	/// Frames per second.
	pub fps: f64,
}

impl GameSpecs {
	pub fn get_tick_rate(&self) -> Result<Duration, TryFromFloatSecsError> {
		Duration::try_from_secs_f64(1.0 / self.tps)
	}

	pub fn get_frame_rate(&self) -> Result<Duration, TryFromFloatSecsError> {
		Duration::try_from_secs_f64(1.0 / self.fps)
	}
}

impl Default for GameSpecs {
	fn default() -> Self {
		Self {
			tps: 16.0,
			fps: 60.0,
		}
	}
}

/// Handler for processing terminal-related events and producing application
/// events. This struct also has [`Deref`] and [`DerefMut`] implementations to
/// the contained [`Station::terminal`]. When this struct is [`Drop`]ped,
/// [`Station::exit`] will be called.
///
/// Note that by default, mouse capture is not enabled.
///
/// This struct provides two methods to influence its control flow:
/// [`Station::start`] and [`Station::stop`] (which gets called when dropping
/// this struct).
///
/// To begin, see [`Station::enter`] and [`Station::exit`] for the recommended
/// ways to start the terminal interface. Typically, only [`Station::enter`]
/// will need to be called after [creating](Station::new) a station object, as
/// dropping it will automatically make it [exit](Station::exit).
#[derive(Debug)]
pub struct Station {
	/// Terminal interface to interact with.
	pub terminal: Terminal,

	/// Handle for event task.
	event_task: JoinHandle<crate::Result<()>>,

	/// This handler's cancellation token.
	pub cancel_token: CancellationToken,

	/// [`StationEvent`] channel.
	pub event_channel: UnboundedChannel<StationEvent>,

	/// Tick rate - how rapidly to update state.
	tick_rate: Duration,

	/// Frame rate - how rapidly to render.
	frame_rate: Duration,
}

impl Station {
	/// Constructs a new terminal interface object with the provided
	/// [`GameSpecs`].
	pub fn new(game_specs: &GameSpecs) -> crate::Result<Self> {
		Ok(Self {
			terminal: Self::get_terminal()?,
			event_task: tokio::spawn(async move { Ok(()) }),
			cancel_token: CancellationToken::new(),
			event_channel: UnboundedChannel::new(),
			tick_rate: game_specs.get_tick_rate()?,
			frame_rate: game_specs.get_frame_rate()?,
		})
	}

	/// Returns an instance of [`Terminal`] this app uses.
	fn get_terminal() -> std::io::Result<Terminal> {
		Terminal::new(CrosstermBackend::new(std::io::stdout()))
	}

	/// Tries to receive the next [station event](StationEvent).
	pub fn try_recv_event(&mut self) -> Result<StationEvent, TryRecvError> {
		self.event_channel.try_recv()
	}

	/// Sends one [station event](StationEvent) to the [event
	/// channel](Self::event_channel).
	fn send_station_event(
		event_sender: &UnboundedSender<StationEvent>,
		station_event: StationEvent,
	) -> crate::Result<()> {
		if station_event.should_be_logged() {
			tracing::debug!(?station_event, "sending station event");
		}
		event_sender.send(station_event)?;
		Ok(())
	}

	/// Event loop to interact with the terminal.
	#[tracing::instrument(
		level = "info",
		name = "terminal-event-loop",
		skip_all
	)]
	async fn event_loop(
		event_sender: UnboundedSender<StationEvent>,
		cancel_token: CancellationToken,
		tick_rate: Duration,
		frame_rate: Duration,
	) -> crate::Result<()> {
		let mut event_stream = CrosstermEventStream::new();
		let mut tick_interval = interval(tick_rate);
		let mut render_interval = interval(frame_rate);

		if let Err(err) = event_sender.send(StationEvent::Hello) {
			return Err(eyre!("while sending greetings! how rude: {err}"));
		}

		loop {
			let station_event = tokio::select! {
				() = cancel_token.cancelled() => {
					tracing::info!("station's cancel token cancelled");
					break;
				},
				() = event_sender.closed() => {
					tracing::info!("event sender closed");
					break;
				},
				_ = tick_interval.tick() => StationEvent::Tick,
				_ = render_interval.tick() => StationEvent::Render,
				crossterm_event = event_stream.next().fuse() => match crossterm_event {
					Some(Ok(event)) => {
						event.into()
					},
					Some(Err(err)) => {
						return Err(eyre!("while receiving from event stream: {err}"));
					},
					None => {
						tracing::warn!("event stream closed; no more events are to be consumed");
						break;
					}
				},
			};
			if let Err(err) =
				Self::send_station_event(&event_sender, station_event.clone())
			{
				return Err(eyre!("while sending station event: {err}")
					.with_note(|| {
						format!("trying to send event: {station_event:?}")
					}));
			}
		}
		tracing::info!("station event loop is finished");
		Ok(())
	}

	/// Begins event reception and enters the terminal.
	#[tracing::instrument(skip(self))]
	pub fn enter(&mut self) -> crate::Result<()> {
		tracing::info!("entering the station");
		Self::set_terminal_rules()?;
		self.start();
		Ok(())
	}

	/// Exits the terminal interface.
	#[tracing::instrument(skip(self))]
	pub fn exit(&mut self) -> crate::Result<()> {
		tracing::info!("exiting the station");
		self.stop()?;
		Self::reset_terminal_rules()?;
		Ok(())
	}

	/// (Re-)starts the terminal interface layer.
	#[tracing::instrument(skip(self))]
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
	#[tracing::instrument(skip(self))]
	pub fn stop(&mut self) -> crate::Result<()> {
		self.cancel_token.cancel();
		let one_ms = Duration::from_millis(1);
		let mut cancel_timer = Duration::from_millis(0);
		let mut aborting = false;

		while !self.event_task.is_finished() {
			std::thread::sleep(one_ms);
			cancel_timer += one_ms;

			if cancel_timer > Duration::from_millis(100) && !aborting {
				tracing::warn!(
					"could not cancel event task thread after 100ms; aborting \
					 it"
				);
				aborting = true;
				self.event_task.abort();
			} else if cancel_timer > Duration::from_millis(200) {
				let message = "could not abort event task thread after 200ms";
				tracing::error!("{message}; exiting");
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
			DisableMouseCapture,
			DisableFocusChange,
			EnableBlinking,
			LeaveAlternateScreen,
			Show,
		)?;
		Ok(())
	}
}

impl Drop for Station {
	fn drop(&mut self) {
		if let Err(err) = self.exit() {
			panic!("could not exit the station (when dropping): {err}");
		}
	}
}
