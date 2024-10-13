//! Metadata for a [screen](Screens).

use derive_builder::Builder;
use unicode_segmentation::UnicodeSegmentation;

use crate::ui::UiRunState;

/// A set of properties that always goes with every instance of a [`Screen`].
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct ScreenData {
	/// Run state of the screen.
	#[builder(field(private))]
	#[builder(default)]
	pub run_state: UiRunState,

	/// Title of the screen.
	pub title: String,

	/// Whether the screen needs mouse input.
	#[builder(default = "false")]
	pub captures_mouse: bool,
}

impl ScreenData {
	/// Returns a new default [`ScreenDataBuilder`].
	pub fn builder() -> ScreenDataBuilder {
		ScreenDataBuilder::default()
	}

	/// Returns a title padded with `padding` on both sides. `padding`
	/// gets reversed on the right side.
	#[expect(unused, reason = "just a fun thing")]
	pub fn get_padded_title<T: ToString>(&self, padding: &T) -> String {
		let padding = padding.to_string();
		let left_pad = padding.clone();
		// See how I add `unicode-segmentation` just for this?
		// Very mindful, very demure.
		let right_pad: String = padding.graphemes(true).rev().collect();
		format!("{}{}{}", left_pad, self.title.clone(), right_pad)
	}
}
