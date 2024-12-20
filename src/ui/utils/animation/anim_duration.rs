//! A duration that specifies time for an animation.

use std::time::Duration;

use crate::ui::utils::wrap_index;

/// Duration that semantically specifies time for an animation.
#[derive(Debug, Clone)]
pub enum AnimationDuration {
	/// Total duration this animation should run for.
	Total(Duration),

	/// Even duration each frame should run for.
	EvenFrame(Duration),

	/// Specifies a duration for each frame to run for.
	/// Note that if the length of the frames and the length of the frames
	/// don't match, index overflow & wrapping is accounted for
	/// [Python-style](super::wrap_index).
	Frames(Vec<Duration>),
}

impl AnimationDuration {
	/// Gets the total duration of the animation, optionally providing a length
	/// if inquiring into the [`AnimationDuration::EvenFrame`] variant.
	pub fn get_total_duration(&self, length: Option<u32>) -> Duration {
		match self {
			Self::Total(duration) => *duration,
			Self::EvenFrame(duration) => {
				assert!(
					length.is_some(),
					"expected a length because looking into `EvenFrame` \
					 variant"
				);
				*duration * length.unwrap()
			},
			Self::Frames(frames) => frames.iter().sum(),
		}
	}

	/// Gets the duration of one frame, optionally at the length if
	/// looking into the [`AnimationDuration::Total`] variant, or the index if
	/// looking into the [`AnimationDuration::Frames`] variant.
	pub fn get_frame_duration(
		&self,
		length: Option<u32>,
		index: Option<isize>,
	) -> Duration {
		match self {
			AnimationDuration::Total(duration) => {
				*duration
					/ length.expect(
						"expected a length because looking into `Total` \
						 variant",
					)
			},
			AnimationDuration::EvenFrame(duration) => *duration,
			AnimationDuration::Frames(frames) => {
				let index = wrap_index(
					index.expect(
						"expected an index because looking into `EachFrames` \
						 variant",
					),
					frames.len(),
				);
				frames[index]
			},
		}
	}
}

#[cfg(test)]
mod test {
	use rstest::*;

	use super::*;

	#[rstest]
	fn total_anim_duration_works() {
		let total = AnimationDuration::Total(Duration::from_secs_f64(20.0));
		assert_eq!(
			total.get_frame_duration(Some(1), None),
			Duration::from_secs_f64(20.0),
			"total should be 20.0s"
		);
		assert_eq!(
			total.get_frame_duration(Some(4), None),
			Duration::from_secs_f64(5.0),
			"total of 20.0s / 4 frames = 5.0s each"
		);
		assert_eq!(
			total.get_frame_duration(Some(1), None),
			total.get_total_duration(None),
			"getting whole duration of total in 2 different ways"
		);
	}

	#[rstest]
	fn even_frame_anim_duration_works() {
		assert_eq!(
			AnimationDuration::EvenFrame(Duration::from_secs_f64(5.0))
				.get_total_duration(Some(10)),
			Duration::from_secs_f64(50.0),
			"5.0s even frames * 10 = 50.0s"
		);
	}

	#[rstest]
	fn frames_anim_duration_works() {
		let frames = AnimationDuration::Frames(
			vec![0.5, 1.0, 2.0, 5.0, 7.5, 10.0]
				.into_iter()
				.map(Duration::from_secs_f64)
				.collect(),
		);
		assert_eq!(
			frames.get_frame_duration(None, Some(3)),
			Duration::from_secs_f64(5.0),
			"frames[3] = 5.0s"
		);
		assert_eq!(
			frames.get_total_duration(None),
			Duration::from_secs_f64(26.0),
			"who knows i might be good at addition",
		);
	}

	#[rstest]
	#[should_panic(
		expected = "expected a length because looking into `EvenFrame` variant"
	)]
	fn total_duration_of_even_frame_without_length_panics() {
		AnimationDuration::EvenFrame(Duration::from_secs(0))
			.get_total_duration(None);
	}

	#[rstest]
	#[should_panic(
		expected = "expected a length because looking into `Total` variant"
	)]
	fn frame_duration_of_total_without_length_panics() {
		AnimationDuration::Total(Duration::from_secs(0))
			.get_frame_duration(None, None);
	}

	#[rstest]
	#[should_panic(expected = "expected an index because looking into \
	                           `EachFrames` variant")]
	fn frame_duration_of_frames_without_index_panics() {
		AnimationDuration::Frames(vec![Duration::from_secs(0)])
			.get_frame_duration(None, None);
	}
}
