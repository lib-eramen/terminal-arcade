//! A collection of individual UI elements and widgets.
//!
//! Note that the UI conventions included in here are somewhat unfaithful to the
//! way [ratatui] works with widgets, especially stateful ones (by separating
//! state from view).

use ratatui::layout::Layout;

pub mod game_select;
pub mod games;
pub mod presets;
pub mod screen_base_block;
pub mod under_construction;
pub mod welcome;
