//! A simple sequencing that runs through the frames, one by one, showing one
//! after another.

use std::time::Duration;

use super::{
	linear::LinearSequencing,
	Sequencing,
};
use crate::ui::utils::animation::AnimationDuration;

/// A simple sequencing that runs through the frames, one by one, showing one
/// after another. This struct wraps a basic [`LinearSequencing`] object.
#[derive(Debug, Clone)]
pub struct SimpleSequencing(LinearSequencing);

impl SimpleSequencing {
	/// Constructs a new simple sequencing object.
	pub fn new(duration: AnimationDuration, length: usize) -> Self {
		let sequencing = LinearSequencing::builder()
			.duration(duration)
			.length(length)
			.interval(1)
			.build()
			.unwrap();
		Self(sequencing)
	}
}

impl<F> Sequencing<F> for SimpleSequencing
where
	F: Clone,
{
	type Data = Vec<F>;

	fn get_displayed_frame(&self, frames: &Self::Data) -> F {
		let index = self.0.get_frame_indices().drain().collect::<Vec<_>>()[0];
		frames[index].clone()
	}

	fn step(&mut self) {
		self.0.step();
	}

	fn step_back(&mut self) {
		self.0.step_back();
	}

	fn should_step(&self, elapsed: Duration) -> bool {
		self.0.should_step(elapsed)
	}
}
