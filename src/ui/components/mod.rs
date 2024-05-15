//! A collection of individual UI elements and widgets.
//!
//! Note that the UI conventions included in here are somewhat unfaithful to the
//! way [ratatui] works with widgets, especially stateful ones (by separating
//! state from view).

use ratatui::layout::Layout;

pub mod flicker_counter;
pub mod game_select;
pub mod games;
pub mod presets;
pub mod scroll_tracker;
pub mod scrollable_list;
pub mod text_input_bar;
pub mod under_construction;
pub mod welcome;
