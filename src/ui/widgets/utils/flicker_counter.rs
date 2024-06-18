//! Module for everything [flicker counters](FlickerCounter).

use std::{
	sync::Mutex,
	time::{
		Duration,
		SystemTime,
	},
};

use lazy_static::lazy_static;

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
	/// The beginning time of this flicker counter.
	pub begin_time: SystemTime,

	/// Interval to toggle.
	pub interval: Duration,
}

impl Default for FlickerCounter {
	fn default() -> Self {
		Self {
			begin_time: SystemTime::now(),
			interval: Duration::from_secs_f32(0.5),
		}
	}
}

impl FlickerCounter {
	/// Creates a new flicker counter, the state initially being
	/// [`FlickerState::On`].
	pub fn new(interval: Duration) -> Self {
		Self {
			begin_time: SystemTime::now(),
			interval,
		}
	}

	/// Updates the counter with the [`SystemTime::now`] time.
	pub fn reset(&mut self) {
		self.begin_time = SystemTime::now();
	}

	/// Gets the current [flicker state](FlickerState).
	pub fn get_state(&self) -> FlickerState {
		let elapsed = self.begin_time.elapsed().expect("Time is not making sense").as_nanos();
		if elapsed / self.interval.as_nanos() % 2 == 0 {
			FlickerState::On
		} else {
			FlickerState::Off
		}
	}

	/// Checks if this counter's state is [`FlickerState::On`].
	#[must_use]
	pub fn is_on(&self) -> bool {
		self.get_state() == FlickerState::On
	}

	/// Checks if this counter's state is [`FlickerState::Off`].
	#[must_use]
	pub fn is_off(&self) -> bool {
		self.get_state() == FlickerState::Off
	}
}
