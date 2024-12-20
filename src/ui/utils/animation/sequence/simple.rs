//! A simple sequencing that runs through the frames, one by one, showing one
//! after another.

use std::time::Duration;

use super::Sequencing;
use crate::ui::utils::animation::AnimationDuration;

/// A simple sequencing that runs through the frames, one by one, showing one
/// after another.
#[derive(Debug, Clone)]
pub struct SimpleSequencing {
	/// Duration of this animation (runs until the anchor reaches the end of
	/// the sequence).
	duration: AnimationDuration,

	/// Index of the frame this sequence is currently at.
	anchor: usize,

	/// Length of the sequence.
	length: usize,
}

impl SimpleSequencing {
	/// Constructs a new simple sequencing object.
	pub fn new(duration: AnimationDuration, length: usize) -> Self {
		debug_assert_ne!(length, 0, "length should not be zero");
		Self {
			duration,
			anchor: 0,
			length,
		}
	}
}

impl<F> Sequencing<F> for SimpleSequencing
where
	F: Clone,
{
	type Data = Vec<F>;

	fn get_displayed_frame(&self, frames: &Self::Data) -> F {
		frames[self.anchor].clone()
	}

	fn step(&mut self) {
		self.anchor = if self.anchor == self.length - 1 {
			0
		} else {
			self.anchor + 1
		};
	}

	fn step_back(&mut self) {
		self.anchor = if self.anchor == 0 {
			self.length - 1
		} else {
			self.anchor - 1
		};
	}

	fn should_step(&self, elapsed: Duration) -> bool {
		let length =
			u32::try_from(self.length).expect("length does not fit in `u32`");
		let index = isize::try_from(self.anchor)
			.expect("anchor does not fit in `isize`");
		elapsed >= self.duration.get_frame_duration(Some(length), Some(index))
	}
}
