//! Handler to manage screens and rendering them.

use serde::{
	Deserialize,
	Serialize,
};

use crate::ui::screen::ScreenHandle;

/// Handler for screens.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScreenHandler {
	/// A stack of screens.
	///
	/// The top most screen (last element) renders and receives events.
	stack: Vec<ScreenHandle>,
}
