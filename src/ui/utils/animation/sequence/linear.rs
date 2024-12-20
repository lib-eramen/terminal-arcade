//! Linear sequencing that has useful methods for combining frames for banners.

use std::{
	collections::HashSet,
	time::Duration,
};

use super::Sequencing;
use crate::ui::utils::{
	animation::{
		AnimationDuration,
		FrameCombinator,
	},
	wrap_index,
};

/// Linear sequence of (sets of offsetted) indices. The default is a sequence
/// that steps through each frame, left to right.
#[derive(Debug, Clone, derive_builder::Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct LinearSequencing {
	/// Frame combinator.
	combinator: FrameCombinator,

	/// Duration of this animation (runs until the anchor reaches the end of
	/// the sequence).
	duration: AnimationDuration,

	/// An index-like number to generate frames off of.
	#[builder(default = "0", setter(skip))]
	anchor: usize,

	/// Length of the sequence.
	length: usize,

	/// Interval to step through the frames at. By default, it is 1—each frame
	/// is stepped through, left to right. An interval of -1 means it goes in
	/// reverse.
	interval: isize,

	/// Offsets to the main anchor point on the sequence.
	///
	/// Note that if this option is set, only the offsetted
	/// indices will be generated—including the frame on the anchor also means
	/// placing a 0 offset somewhere in this [`Vec`].
	#[builder(default, setter(strip_option))]
	offsets: Option<HashSet<isize>>,
}

impl LinearSequencingBuilder {
	/// Asserts that the length is not zero.
	#[expect(
		clippy::unnecessary_wraps,
		reason = "derive_builder needs result return type"
	)]
	fn validate(&self) -> Result<(), String> {
		debug_assert_ne!(
			self.length,
			Some(0),
			"length should not be absent or zero"
		);
		Ok(())
	}
}

impl LinearSequencing {
	/// Returns a default builder instance of this struct.
	pub fn builder() -> LinearSequencingBuilder {
		LinearSequencingBuilder::default()
	}

	/// Returns the current frame indices that points to the current
	/// [`Self::anchor`].
	pub fn get_frame_indices(&self) -> HashSet<usize> {
		let anchor = wrap_index(
			self.anchor.try_into().expect("anchor overflowed"),
			self.length,
		);
		match &self.offsets {
			Some(offsets) => offsets
				.iter()
				.map(|offset| {
					wrap_index(
						isize::try_from(anchor).expect("anchor overflowed")
							+ *offset,
						self.length,
					)
				})
				.collect(),
			None => [anchor].into(),
		}
	}
}

impl LinearSequencing {
	/// Validates the `indices` parameter for [`Self::produce_frame`].
	fn validate_indices(&self, indices: &HashSet<usize>) {
		debug_assert!(!indices.is_empty(), "retrieving no frames");
		debug_assert!(
			indices.len() <= self.length,
			"cannot combine more frames than what is given"
		);
	}

	/// Combine frames from indices.
	fn produce_frame(
		&self,
		indices: &HashSet<usize>,
		frames: &[String],
	) -> String {
		indices
			.iter()
			.map(|index| frames[*index].clone())
			.reduce(|earlier, later| {
				earlier
					.chars()
					.zip(later.chars())
					.map(|(earlier, later)| {
						self.combinator.combine(earlier, later)
					})
					.collect::<String>()
			})
			.expect("frame should be non-empty")
	}
}

impl Sequencing<String> for LinearSequencing {
	type Data = Vec<String>;

	fn get_displayed_frame(&self, frames: &Self::Data) -> String {
		let indices = self.get_frame_indices();
		self.validate_indices(&indices);
		self.produce_frame(&indices, frames)
	}

	fn step(&mut self) {
		self.anchor = wrap_index(
			isize::try_from(self.anchor).expect("anchor overflowed")
				+ self.interval,
			self.length,
		);
	}

	fn step_back(&mut self) {
		self.anchor = wrap_index(
			isize::try_from(self.anchor).expect("anchor overflowed")
				- self.interval,
			self.length,
		);
	}

	fn should_step(&self, elapsed: Duration) -> bool {
		let length =
			u32::try_from(self.length).expect("length does not fit in `u32`");
		let index = isize::try_from(self.anchor)
			.expect("anchor does not fit in `isize`");
		elapsed >= self.duration.get_frame_duration(Some(length), Some(index))
	}
}

#[cfg(test)]
mod test {
	use rstest::*;

	use super::*;
	use crate::ui::utils::{
		animation::MismatchCombineMethod,
		wrap_index,
	};

	impl LinearSequencing {
		/// [`Self::step`], but only the anchoring updates happen and the
		/// indices from [`Self::get_frame_indices`] are returned. Only really
		/// useful for testing.
		fn test_step(&mut self) -> HashSet<usize> {
			self.step();
			self.get_frame_indices()
		}
	}

	#[rstest]
	fn wrap_index_actually_works() {
		assert_eq!(wrap_index(0, 5), 0);
		assert_eq!(wrap_index(2, 5), 2);
		assert_eq!(wrap_index(8, 5), 3);
		assert_eq!(wrap_index(100, 5), 0);
		assert_eq!(wrap_index(-1, 5), 4);
		assert_eq!(wrap_index(-4, 5), 1);
		assert_eq!(wrap_index(-5, 5), 0);
		assert_eq!(wrap_index(-100, 5), 0);
	}

	#[fixture]
	fn sequencing_builder() -> LinearSequencingBuilder {
		let mut sequencing = LinearSequencing::builder();
		sequencing
			.combinator(FrameCombinator::new(
				MismatchCombineMethod::EnforceDiscard,
				vec![],
			))
			.duration(AnimationDuration::Total(Duration::from_secs(1)));
		sequencing
	}

	#[fixture]
	fn combinator() -> FrameCombinator {
		FrameCombinator::new(MismatchCombineMethod::EnforceDiscard, vec![])
	}

	#[rstest]
	fn linear_sequencing_works(
		mut sequencing_builder: LinearSequencingBuilder,
	) {
		let mut sequencing =
			sequencing_builder.length(5).interval(1).build().unwrap();

		for i in 0..4 {
			let step_to = i + 1;
			assert_eq!(
				sequencing.test_step(),
				[step_to].into(),
				"linear sequence should step to {step_to}",
			);
		}

		assert_eq!(
			sequencing.test_step(),
			[0].into(),
			"linear sequence should overflow back to 0",
		);
		assert_eq!(
			sequencing.test_step(),
			[1].into(),
			"after overflow, linear sequence should continue to 1",
		);
	}

	#[rstest]
	fn linear_sequencing_with_negative_interval_works_backwards(
		mut sequencing_builder: LinearSequencingBuilder,
	) {
		let mut sequencing =
			sequencing_builder.length(5).interval(-1).build().unwrap();

		for i in 0..5 {
			let step_back_to = 4 - i;
			assert_eq!(
				sequencing.test_step(),
				[step_back_to].into(),
				"linear sequence should step back to {step_back_to}",
			);
		}

		assert_eq!(
			sequencing.test_step(),
			[4].into(),
			"linear sequence should overflow back to 4",
		);
	}

	#[rstest]
	fn linear_sequencing_with_interval_greater_than_length_works(
		mut sequencing_builder: LinearSequencingBuilder,
	) {
		let mut sequencing =
			sequencing_builder.length(5).interval(7).build().unwrap();

		assert_eq!(
			sequencing.test_step(),
			[2].into(),
			"linear sequence should step 7 units to 2",
		);
		assert_eq!(
			sequencing.test_step(),
			[4].into(),
			"linear sequence should step 7 more units to 4",
		);
	}

	#[rstest]
	fn linear_sequencing_with_interval_1_sets_anchor_to_result(
		#[values(1, 3, 10, 50, 100)] steps: usize,
		mut sequencing_builder: LinearSequencingBuilder,
	) {
		let mut step_to = HashSet::new();
		let mut sequencing =
			sequencing_builder.length(10).interval(1).build().unwrap();

		for _ in 0..steps {
			step_to = sequencing.test_step();
		}

		assert_eq!(
			HashSet::from([sequencing.anchor]),
			step_to,
			"anchor should be where the step is for 1-interval linear \
			 sequencing"
		);
	}

	#[rstest]
	fn linear_sequencing_with_length_1_steps_in_place(
		#[values(-5, -1, 0, 1, 5, 100)] interval: isize,
		mut sequencing_builder: LinearSequencingBuilder,
	) {
		let mut sequencing =
			sequencing_builder.length(1).interval(1).build().unwrap();

		for _ in 0..3 {
			assert_eq!(
				sequencing.test_step(),
				[0].into(),
				"linear sequence with interval {interval} and length 1 should \
				 always remain at 0",
			);
			assert_eq!(
				sequencing.anchor, 0,
				"anchor should always remain at 0 at length 1"
			);
		}
	}
	#[fixture]
	fn offsets() -> HashSet<isize> {
		HashSet::from([-1, 0, 2])
	}

	#[rstest]
	fn offsetted_linear_sequencing_with_interval_1_works(
		offsets: HashSet<isize>,
		mut sequencing_builder: LinearSequencingBuilder,
	) {
		let mut sequencing = sequencing_builder
			.length(5)
			.interval(1)
			.offsets(offsets)
			.build()
			.unwrap();

		assert_eq!(
			sequencing.test_step(),
			[0, 1, 3].into(),
			"linear sequencing works!",
		);
		assert_eq!(
			sequencing.test_step(),
			[1, 2, 4].into(),
			"linear sequencing works! idk what else to say",
		);
		assert_eq!(
			sequencing.test_step(),
			[2, 3, 0].into(),
			"linear sequencing works and handles overflowing as well! how \
			 multitalented", /* is this worth sacrificing my sleep schedule
			                  * over? */
		);
	}

	#[rstest]
	fn offsetted_linear_sequencing_with_interval_4_works(
		offsets: HashSet<isize>,
		mut sequencing_builder: LinearSequencingBuilder,
	) {
		let mut sequencing = sequencing_builder
			.length(5)
			.interval(4)
			.offsets(offsets)
			.build()
			.unwrap();

		assert_eq!(
			sequencing.test_step(),
			[3, 4, 1].into(),
			"linear sequencing works! also modulo arithmetic is so difficult",
		);
		assert_eq!(
			sequencing.test_step(),
			[2, 3, 0].into(),
			"linear sequencing works! how do i say that in a different way",
		);
		// i don't have the mental capacity to process a third step's case
	}

	#[rstest]
	fn offsetted_linear_sequencing_with_interval_neg_3_works(
		offsets: HashSet<isize>,
		mut sequencing_builder: LinearSequencingBuilder,
	) {
		let mut sequencing = sequencing_builder
			.length(5)
			.interval(-3)
			.offsets(offsets)
			.build()
			.unwrap();

		assert_eq!(
			sequencing.test_step(),
			[1, 2, 4].into(),
			"linear sequencing works... finally",
		);
		assert_eq!(
			sequencing.test_step(),
			[3, 4, 1].into(),
			"one can sense the palpable anxiety with this one",
		);
		// please save me i've been on this file for 3 hours
	}
}
