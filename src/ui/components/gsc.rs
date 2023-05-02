use ansi_to_tui::IntoText;
use ratatui::{text::Text, layout::{Layout, Direction, Constraint}};

use crate::ui::util::{stylize, stylize_raw};

/// Returns the text used in the back "button".
#[must_use]
pub fn back_button_text() -> Text<'static> {
    stylize("← Back ([ESC])")
}

/// Returns the default text used in the search bar.
#[must_use]
pub fn search_bar_default_text() -> Text<'static> {
    stylize("🔎︎ Search... ([CTRL-Backspace to clear]")
}

/// Returns the I'm Feeling Lucky help text.
#[must_use]
pub fn im_feeling_lucky_text() -> Text<'static> {
    format!(
        "Feeling {}? {} for a {} game!",
        stylize_raw("lucky"),
        stylize_raw("[Ctrl-R]"),
        stylize_raw("random"),
    ).into_text().unwrap()
}

/// Returns the layout for a search bar.
#[must_use]
pub fn search_bar_layout() -> Layout {
    Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Ratio(1, 8), // Back "button"
            Constraint::Ratio(7, 8), // Search area
        ])
}

/// Returns the layout for the general search bar section.
#[must_use]
pub fn search_section_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Max(3), // Back "button" and search bar
            Constraint::Max(3), // I'm Feeling Lucky help text
            Constraint::Max(0), // Prevent blocks from taking up remaining space
        ])
}
