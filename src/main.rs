//! # Terminal Arcade
//!
//! Terminal-based arcade-style games for when you're bored out of your mind.
//!
//! Expect this to be a work-in-progress always! New games and features and
//! to-be-debugged spaghetti code guaranteed.

#![forbid(unsafe_code)]
#![deny(missing_docs, clippy::suspicious, clippy::todo)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]
#![allow(
	clippy::module_name_repetitions,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc
)]

mod app;
mod config;
mod events;
mod services;
mod tui;
mod ui;
mod utils;

use app::App;
use config::Config;
use services::files::AppFiles;

/// Result type for the entire crate. Uses [`color_eyre`]'s
/// [Result](color_eyre::eyre::Result) type.
type Result<T, E = color_eyre::eyre::Report> = color_eyre::eyre::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
	let app_files = AppFiles::default();
	services::initialize_services(&app_files)?;
	let config = Config::fetch(app_files)?;
	App::new(config)?.run()
}
