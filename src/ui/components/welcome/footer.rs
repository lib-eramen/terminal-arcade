//! A bottom bar for the welcome screen.

use ratatui::{
	layout::{
		Alignment,
		Rect,
	},
	text::Text,
	widgets::{
		Paragraph,
		Wrap,
	},
	Frame,
};

use crate::ui::{
	components::presets::untitled_ui_block,
	util::get_crate_version,
};

// TODO: Somehow the footer needs to be turned into a Paragraph and wrapped
// properly

#[must_use]
fn git_info_string() -> String {
	let git_info = git_info::get();
	let current_branch =
		git_info.current_branch.unwrap_or_else(|| "of an unknown tree".to_string());
	let version = get_crate_version();
	let commit_hash = git_info.head.last_commit_hash_short.unwrap_or_else(|| "browns".to_string());
	let remote_link = "https://github.com/developer-ramen/terminal-arcade";

	format!(
		"🎮 Terminal Arcade {version}, on 🎋 {current_branch}, commit hash {commit_hash}; at \
		 remote {remote_link}",
	)
}

#[must_use]
fn bottom_bar_text() -> String {
	format!(
		"⏰ Time: {}\n{}\n🏗️ Terminal Arcade is a work-in-progress! If you would like to \
		 contribute, please do!
        ",
		chrono::Local::now().format("%d/%m/%Y %H:%M:%S"),
		git_info_string(),
	)
}

/// Renders the bottom bar at the welcome screen.
pub fn render_welcome_bottom_bar(frame: &mut Frame<'_>, size: Rect) {
	let bottom_bar_paragraph = Paragraph::new(bottom_bar_text())
		.alignment(Alignment::Center)
		.wrap(Wrap { trim: true })
		.block(untitled_ui_block());
	frame.render_widget(bottom_bar_paragraph, size);
}
