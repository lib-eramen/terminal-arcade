//! # Terminal Arcade
//!
//! Terminal-based arcade-style games for when you're bored out of your mind.
//!
//! Expect this to be a work-in-progress always! New games and features and
//! to-be-debugged spaghetti code guaranteed.

#![forbid(unsafe_code)]
#![deny(
	missing_docs,
	clippy::suspicious,
	clippy::unwrap_used,
	clippy::expect_used
)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use color_eyre::Section;

use crate::{
	app::App,
	config::Config,
	tui::Tui,
};

mod app;
mod components;
mod config;
mod events;
mod services;
mod tui;
mod ui;
mod utils;

/// Result type for the entire crate. Uses [`color_eyre`]'s
/// [Result](color_eyre::eyre::Result) type.
type Result<T, E = color_eyre::eyre::Report> = color_eyre::eyre::Result<T, E>;

fn run() -> Result<()> {
	services::initialize_services()?;
	let config = Config::fetch()?;
	let tui = Tui::with_specs(config.game_specs)?;
	App::default().run(tui, config)
}

#[tokio::main]
async fn main() -> Result<()> {
	if let Err(err) = run() {
		Err(err
			.wrap_err("oh no! something went unhandled!")
			.note("someone get me a paper bag PRONTO")
			.with_section(|| services::oops::ERROR_MSG.clone()))
	} else {
		println!("See you next time! 🕹️ 👋");
		Ok(())
	}
}
