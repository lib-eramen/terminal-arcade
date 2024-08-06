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

use color_eyre::Section;

use crate::{
	app::App,
	config::Config,
	service::{
		errors::ERROR_MSG,
		initialize_utils,
	},
	tui::Tui,
};

mod app;
mod config;
mod event;
mod service;
mod tui;
mod ui;
mod utils;

/// Result type for the entire crate. Uses [`color_eyre`]'s
/// [Result](color_eyre::eyre::Result) type.
type Result<T, E = color_eyre::eyre::Report> = color_eyre::eyre::Result<T, E>;

async fn run() -> Result<()> {
	initialize_utils()?;
	let config = Config::fetch()?;
	let tui = Tui::with_specs(config.game_specs)?;
	App::with_config(config).run(tui).await?;
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	if let Err(err) = run().await {
		Err(err
			.wrap_err("an error happened!")
			.note("someone get me a paper bag PRONTO")
			.with_section(|| ERROR_MSG.clone()))
	} else {
		println!("See you next time! 🕹️ 👋");
		Ok(())
	}
}
