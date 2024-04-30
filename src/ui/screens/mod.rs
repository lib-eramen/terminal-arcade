//! Module for screens used in Terminal Arcade.

pub mod config;
pub mod game_select;
pub mod games;
pub mod welcome;

pub use config::ConfigScreen;
pub use game_select::GameSelectionScreen;
pub use games::*;
pub use welcome::WelcomeScreen;

use super::components::presets::titled_ui_block;
