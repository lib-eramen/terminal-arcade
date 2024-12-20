//! Simple presets for [`Block`] containers.

use ratatui::{
	style::{
		Color,
		Style,
	},
	widgets::{
		block::Title,
		Block,
		BorderType,
		Borders,
	},
};

/// A default, untitled block template:
/// * Borders on all sides
/// * Dark gray, rounded borders.
/// * Dark gray foreground
/// * Uniform 1 padding
pub fn untitled_block<'a>() -> Block<'a> {
	Block::default()
		.borders(Borders::ALL)
		.border_type(BorderType::Rounded)
		.style(Style::default().fg(Color::White))
}

/// A block with a centered title, built on top of an [`untitled_block`].
pub fn titled_block<'a, T: Into<Title<'a>>>(title: T) -> Block<'a> {
	untitled_block().title(title)
}
