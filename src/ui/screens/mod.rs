//! Module for screens used in Terminal Arcade.

pub mod config;
pub mod controls_popup;
pub mod game_select;
pub mod games;
pub mod welcome;

pub use config::ConfigScreen;
pub use controls_popup::ControlsPopup;
pub use game_select::GameSearchScreen;
pub use games::*;
pub use welcome::WelcomeScreen;

use super::components::presets::titled_ui_block;
