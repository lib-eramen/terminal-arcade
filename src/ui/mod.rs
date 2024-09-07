//! User interface structures in Terminal Arcade.

use serde::{
	Deserialize,
	Serialize,
};

pub mod screens;

/// Running state of any given UI moving part (a screen, widget) that runs and
/// closes.
///
/// "Close" here is used with a nuance - some other code is run or action is
/// expected to be done before the part is ready to be closed. As such,
/// "closing" does not apply when the part is being forcibly quit.
#[derive(
	Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
#[allow(missing_docs)] // Relatively obvious variant names
pub enum UiRunState {
	/// The part is running.
	#[default]
	Running,

	/// The part is closing and is not forced to immediately quit.
	Closing,

	/// The part has finished closing.
	Finished,
}
