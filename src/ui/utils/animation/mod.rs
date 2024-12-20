//! Text frame-based primitives for animation. Does not follow
//! any actual animation specs or guidelines.

use std::{
	fmt::Debug,
	marker::PhantomData,
	time::Instant,
};

use crossterm::event::{
	KeyCode,
	KeyModifiers,
};
use ratatui::{
	prelude::{
		Buffer,
		Rect,
		Style,
	},
	style::Styled,
	widgets::Block,
};

use self::state::AnimationState;
use crate::{
	events::{
		Event,
		InputEvent,
	},
	ui::UiElement,
};

pub mod anim_duration;
pub mod combine;
pub mod sequence;
pub mod state;

pub use anim_duration::*;
pub use combine::*;
pub use state::*;

/// Enables [`Animation`]s to provide custom rendering behavior.
pub trait FrameRender: Debug + Send + Sync {
	/// State to render with.
	type State;

	/// Renders a frame.
	fn render_frame(
		&self,
		state: &Self::State,
		style: Style,
		buffer: &mut Buffer,
		area: Rect,
	);
}

/// A text-based animation.
#[derive(Debug, derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Animation<'a, D, F>
where
	D: Clone,
	F: Clone,
{
	/// Block to hold the animation in.
	#[builder(default, setter(strip_option))]
	block: Option<Block<'a>>,

	/// Data for the animation.
	data: D,

	/// Style of the animation.
	#[builder(default)]
	style: Style,

	/// Custom renderer for animation.
	renderer: Box<dyn FrameRender<State = AnimationState<D, F>>>,

	#[builder(default, setter(skip))]
	phan: PhantomData<F>,
}

impl<'a, D, F> Animation<'a, D, F>
where
	D: Clone,
	F: Clone,
{
	/// Returns a builder for an animation object.
	pub fn builder() -> AnimationBuilder<'a, D, F> {
		AnimationBuilder::default()
	}
}

impl<D, F> Styled for Animation<'_, D, F>
where
	D: Clone,
	F: Clone,
{
	type Item = Self;

	fn style(&self) -> Style {
		self.style
	}

	fn set_style<S: Into<Style>>(mut self, style: S) -> Self::Item {
		self.style = style.into();
		self
	}
}

impl<D, F> UiElement for Animation<'_, D, F>
where
	D: Clone,
	F: Clone,
{
	type State = AnimationState<D, F>;

	fn tick(&mut self, state: &mut Self::State) -> crate::Result<()> {
		let elapsed = state.last_stepped.elapsed();
		if state.sequencing.should_step(elapsed) {
			state.sequencing.step();
			state.last_stepped = Instant::now();
			state.current_frame =
				state.sequencing.get_displayed_frame(&self.data);
		}
		Ok(())
	}

	fn event(
		&mut self,
		state: &mut Self::State,
		event: &Event,
	) -> crate::Result<()> {
		if let Event::Input(InputEvent::Key(key)) = event {
			if key.modifiers != KeyModifiers::empty() {
				return Ok(());
			}
			match key.code {
				KeyCode::Char(' ') => state.run_state.toggle(),
				KeyCode::Right => state.sequencing.step(),
				KeyCode::Left => state.sequencing.step_back(),
				_ => {},
			}
		}
		Ok(())
	}

	fn render(&self, state: &Self::State, buffer: &mut Buffer, area: Rect) {
		self.renderer.render_frame(state, self.style, buffer, area);
	}
}
