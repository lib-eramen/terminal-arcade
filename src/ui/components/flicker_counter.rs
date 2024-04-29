//! Module for everything [flicker counters](FlickerCounter).

use std::{
	sync::Mutex,
	time::{
		Duration,
		Instant,
	},
};

use lazy_static::lazy_static;

lazy_static! {
	/// The global flicker counter with a 0.5s interval.
	/// Use this instance and update this instance only if you want flicker effects
	/// to sync together.
	pub static ref GLOBAL_FLICKER_COUNTER: Mutex<FlickerCounter> = Mutex::new(FlickerCounter::new(Duration::from_secs_f32(0.5)));
}

/// A flicker state.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum FlickerState {
	On,
	Off,
}

impl FlickerState {
	/// Toggles to the other state.
	pub fn toggle(&self) -> Self {
		match self {
			FlickerState::On => FlickerState::Off,
			FlickerState::Off => FlickerState::On,
		}
	}
}

/// A flicker counter, used to emulate flickering effects by depending on the
/// system time.
#[derive(Debug, Clone)]
#[must_use]
pub struct FlickerCounter {
	/// Last time recorded.
	pub last_time: Instant,

	/// Interval to toggle.
	pub interval: Duration,

	/// State of the flicker.
	pub state: FlickerState,
}

impl FlickerCounter {
	/// Creates a new flicker counter, the state initially being
	/// [`FlickerState::Appearing`].
	pub fn new(interval: Duration) -> Self {
		Self {
			last_time: Instant::now(),
			interval,
			state: FlickerState::On,
		}
	}

	/// Updates the counter. If the time is past the interval, the time is
	/// always updated to [`Instant::now`].
	///
	/// Intended to be called as often as possible (preferably every frame).
	pub fn update(&mut self) {
		self.update_with_time(Instant::now());
	}

	/// Updates the counter with the [`Instant::now`] time.
	pub fn update_by_now(&mut self) {
		self.last_time = Instant::now();
		self.state = FlickerState::On;
		self.update();
	}

	/// [Updates the counter](update) with a given global time.
	pub fn update_with_time(&mut self, time: Instant) {
		if time - self.last_time >= self.interval {
			self.last_time = time;
			self.state = self.state.toggle();
		}
	}

	/// Checks if this counter's state is [`FlickerState::On`].
	#[must_use]
	pub fn is_on(&self) -> bool {
		self.state == FlickerState::On
	}

	/// Checks if this counter's state is [`FlickerState::Off`].
	#[must_use]
	pub fn is_off(&self) -> bool {
		self.state == FlickerState::Off
	}
}
