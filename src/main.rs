//! # Terminal Arcade
//!
//! Terminal-based arcade-style games for when you're bored out of your mind.
//!
//! Expect this to be a work-in-progress always! New games and features and
//! to-be-debugged spaghetti code guaranteed.

#![forbid(unsafe_code)]
#![deny(missing_docs, clippy::suspicious)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use tracing::instrument;

use crate::{
	app::App,
	config::Config,
	service::initialize_utils,
	tui::Tui,
};

mod app;
mod config;
mod event;
mod service;
mod tui;
mod ui;
mod util;

/// Result type for the entire crate. Uses [`color_eyre`]'s
/// [Result](color_eyre::eyre::Result) type.
type Result<T, E = color_eyre::eyre::Report> = color_eyre::eyre::Result<T, E>;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
	initialize_utils()?;
	let config = Config::fetch()?;
	let tui = Tui::with_specs(config.game_specs)?;
	App::with_config(config).run(tui).await?;
	println!("See you next time! 🕹️ 👋");
	Ok(())
}
