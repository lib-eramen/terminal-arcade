//! An empty base block with bolded borders and a bolded + italicized title for
//! screens to continue drawing on.

use ratatui::{
	style::{Color, Style},
	widgets::Block,
};

use crate::ui::components::presets::{titled_ui_block, HIGHLIGHTED};

/// An empty base block with bolded borders and a bolded + italicized title for
/// screens to continue drawing on.
pub fn screen_base_block<T: ToString>(title: T) -> Block<'static> {
	titled_ui_block(title).border_style(Style::new().fg(Color::White)).title_style(HIGHLIGHTED)
}
