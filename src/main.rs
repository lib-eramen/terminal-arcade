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

use crate::{
	config::Config,
	tui::Tui,
};

mod config;
mod tui;
mod util;

/// Result type for the entire crate. Uses [`color_eyre`]'s
/// [Result](color_eyre::eyre::Result) type.
type Result<T, E = color_eyre::eyre::Report> = color_eyre::eyre::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
	util::initialize_utils()?;
	let _config = Config::new()?;
	let mut tui = Tui::new(10, 60)?;
	tui.enter()?;
	std::thread::sleep(std::time::Duration::from_secs(5));
	drop(tui);
	println!("See you next time! 🕹️ 👋");
	Ok(())
}
