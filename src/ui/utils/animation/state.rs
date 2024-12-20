//! State for an [`Animation`](super::Animation).

use std::time::Instant;

use super::sequence::Sequencing;

/// Run state of an animation—running or paused.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AnimationRunState {
	#[default]
	Running,
	Paused,
}

impl AnimationRunState {
	/// Toggles the run state.
	pub fn toggle(&mut self) {
		*self = match self {
			Self::Running => Self::Paused,
			Self::Paused => Self::Running,
		};
	}
}

/// State for an animation.
#[derive(Debug)]
pub struct AnimationState<D, F>
where
	D: Clone,
	F: Clone,
{
	/// Running state of the animation.
	pub run_state: AnimationRunState,

	/// Current frame of the animation.
	pub current_frame: F,

	/// The last time this animation has stepped.
	pub last_stepped: Instant,

	/// Sequencing of the animation.
	pub sequencing: Box<dyn Sequencing<F, Data = D>>,
}

impl<D, F> AnimationState<D, F>
where
	D: Clone,
	F: Clone + Default,
{
	/// Constructs a new animation state object with the provided sequencing.
	pub fn new<S>(sequencing: S, current_frame: Option<F>) -> Self
	where
		S: Sequencing<F, Data = D> + 'static,
	{
		Self {
			run_state: AnimationRunState::Running,
			current_frame: current_frame.unwrap_or_default(),
			last_stepped: Instant::now(),
			sequencing: Box::new(sequencing),
		}
	}
}
