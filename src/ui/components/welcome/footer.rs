//! A bottom bar for the welcome screen.

use ansi_to_tui::IntoText;
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

use crate::{
	core::terminal::BackendType,
	ui::{
		components::presets::untitled_ui_block,
		util::{
			get_crate_version,
			stylize_raw,
		},
	},
};

#[must_use]
fn git_info_string() -> String {
	let git_info = git_info::get();
	let current_branch =
		git_info.current_branch.unwrap_or_else(|| "of an unknown tree".to_string());
	let version = get_crate_version();
	let commit_hash = git_info.head.last_commit_hash_short.unwrap_or_else(|| "browns".to_string());
	let remote_link = "https://github.com/developer-ramen/terminal-arcade";

	format!(
		"Terminal Arcade {}, on branch {}, commit hash {}; at remote {}",
		stylize_raw(version),
		stylize_raw(current_branch),
		stylize_raw(commit_hash),
		stylize_raw(remote_link)
	)
}

#[must_use]
fn bottom_bar_text() -> Text<'static> {
	format!(
		"Time: {}\n{}\n{} is a {}! If you would like to contribute, please do!
        ",
		stylize_raw(chrono::Local::now().format("%d/%m/%Y %H:%M:%S").to_string()),
		git_info_string(),
		stylize_raw("Terminal Arcade"),
		stylize_raw("work-in-progress")
	)
	.into_text()
	.unwrap()
}

/// Renders the bottom bar at the welcome screen.
pub fn render_welcome_bottom_bar(frame: &mut Frame<'_>, size: Rect) {
	let bottom_bar_paragraph = Paragraph::new(bottom_bar_text())
		.alignment(Alignment::Center)
		.wrap(Wrap { trim: true })
		.block(untitled_ui_block());
	frame.render_widget(bottom_bar_paragraph, size);
}
