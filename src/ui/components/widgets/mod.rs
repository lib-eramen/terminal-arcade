//! All widgets implemented in Terminal Arcade.

use ratatui::style::{
	Color,
	Modifier,
	Style,
};

pub mod blocks;

/// [`Style`] for "highlighted" text. For now, the highlighted foreground is
/// blue.
pub const HIGHLIGHTED: Style = Style::new()
	.add_modifier(Modifier::BOLD)
	.add_modifier(Modifier::ITALIC)
	.add_modifier(Modifier::UNDERLINED)
	.fg(Color::Blue);
