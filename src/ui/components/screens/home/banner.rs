//! Banner showing the Terminal Arcade ASCII art logo.

use std::{
	collections::HashSet,
	path::PathBuf,
	time::Duration,
};

use ratatui::{
	prelude::{
		Buffer,
		Rect,
		Style,
		Widget,
	},
	widgets::Paragraph,
};

use crate::{
	events::Event,
	ui::{
		utils::animation::{
			sequence::LinearSequencing,
			state::AnimationState,
			Animation,
			AnimationDuration,
			FrameCombinator,
			FrameRender,
			MismatchCombineMethod,
		},
		UiElement,
	},
};

lazy_static::lazy_static! {
	pub static ref BANNER_ASSET_PATH: PathBuf = PathBuf::from("banners/home");
}

/// Simply renders the banner frame's text without any special effects.
#[derive(Debug)]
struct LazyBannerRenderer;

impl FrameRender for LazyBannerRenderer {
	type State = AnimationState<Vec<String>, String>;

	fn render_frame(
		&self,
		state: &Self::State,
		_style: Style,
		buffer: &mut Buffer,
		area: Rect,
	) {
		let banner_text = Paragraph::new(state.current_frame.clone());
		banner_text.render(area, buffer);
	}
}

/// An animated banner/title card for Terminal Arcade.
#[derive(Debug)]
pub struct Banner<'a>(pub Animation<'a, Vec<String>, String>);

impl<'a> Banner<'a> {
	/// Constructs a new banner.
	pub fn new(frames: Vec<String>) -> Self {
		Self(Self::get_banner_animation(frames))
	}

	fn get_banner_animation(
		frames: Vec<String>,
	) -> Animation<'a, Vec<String>, String> {
		Animation::builder()
			.renderer(Box::new(LazyBannerRenderer))
			.data(frames)
			.build()
			.unwrap()
	}
}

/// State for [`Banner`].
#[derive(Debug)]
pub struct BannerState(pub AnimationState<Vec<String>, String>);

impl BannerState {
	/// Constructs a new banner state.
	pub fn new(original_banner: String, frames_len: usize) -> Self {
		Self(AnimationState::new(
			Self::get_banner_anim_sequencing(frames_len),
			Some(original_banner),
		))
	}

	/// Gets the sequencing of the banner animation.
	fn get_banner_anim_sequencing(frames_len: usize) -> LinearSequencing {
		// SimpleSequencing::new(
		// 	AnimationDuration::Total(Duration::from_secs(5)),
		// 	frames_len,
		// )

		// it took a while to tinker w/ this but below is a linear sequencing
		// that works! make sure to keep the combinator the same as it accounts
		// for how the letter frames were constructed.

		// please i spent far too much time on this animation bs it better be
		// put to good use

		LinearSequencing::builder()
			.combinator(FrameCombinator::new(
				MismatchCombineMethod::EnforceDiscard,
				vec![vec![' '], vec!['_', '‾']],
			))
			.duration(AnimationDuration::Total(Duration::from_secs(5)))
			.length(frames_len)
			.interval(1)
			.offsets(HashSet::from([0, 7]))
			.build()
			.unwrap()
	}
}

impl UiElement for Banner<'_> {
	type State = BannerState;

	fn tick(&mut self, state: &mut Self::State) -> crate::Result<()> {
		self.0.tick(&mut state.0)
	}

	fn event(
		&mut self,
		state: &mut Self::State,
		event: &Event,
	) -> crate::Result<()> {
		self.0.event(&mut state.0, event)
	}

	fn render(&self, state: &Self::State, buffer: &mut Buffer, area: Rect) {
		self.0.render(
			&state.0,
			buffer,
			area.inner(ratatui::prelude::Margin {
				horizontal: 4,
				vertical: 2,
			}),
		);
	}
}
