//! Sequencing for the frames.

use std::{
	fmt::Debug,
	time::Duration,
};

pub mod linear;
pub mod simple;

pub use linear::*;
pub use simple::*;

/// Generate a sequence of frame indices.
///
/// Note that the sequence generation should be able to be infinitely
/// called many times and also handle wrapping index values.
pub trait Sequencing<F>: Debug + Send + Sync {
	/// The type of data the sequencing works with.
	type Data;

	/// Gets the frame to be displayed.
	fn get_displayed_frame(&self, data: &Self::Data) -> F;

	/// Updates the sequence one step forward.
	fn step(&mut self);

	/// Updates the sequence one step back.
	fn step_back(&mut self);

	/// Updates the sequence by `n` steps.
	#[expect(unused, reason = "this might be helpful one day idk")]
	fn step_n(&mut self, n: isize) {
		match n {
			0 => {},
			1.. => {
				for _ in 0..n {
					self.step();
				}
			},
			..0 => {
				for _ in n..0 {
					self.step_back();
				}
			},
		}
	}

	/// Determines whether the sequence should step, based on the amount of
	/// elapsed time.
	fn should_step(&self, elapsed: Duration) -> bool;
}
