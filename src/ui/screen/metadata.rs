//! Metadata for a [screen](Screens).

use serde::{
	Deserialize,
	Serialize,
};

use crate::ui::screen::Screen;

/// A set of properties that always goes with every instance of a [`Screen`].
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenState {}

/// Wrapper struct for a [screen](Screens) and its [metadata](ScreenMetadata).
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenWithState {
	screen: Screen,
	metadata: ScreenState,
}
