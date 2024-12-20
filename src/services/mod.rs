//! Services for the backend application side of Terminal Arcade, such as
//! [logging](log), [error and panic handling](panic), [directories](dirs), etc.

use color_eyre::eyre::Context;
use time::{
	format_description::well_known::Iso8601,
	OffsetDateTime,
};

use crate::services::files::AppFiles;

pub mod files;
pub mod log;
pub mod oops;

lazy_static::lazy_static! {
	/// This package's name.
	pub static ref CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

	/// This crate/app/project's name in `SCREAMING_SNAKE_CASE`.
	pub static ref PROJECT_NAME: String =
		CARGO_PKG_NAME.replace('-', "_").to_uppercase();

	/// The timestamp of when Terminal Arcade was run.
	pub static ref RUN_TIMESTAMP: OffsetDateTime = OffsetDateTime::now_utc();
}

/// Checks if `debug_assertions` is on and returns the `debug` parameter if yes,
/// `other` otherwise.
fn debug_either<T>(debug: T, other: T) -> T {
	if cfg!(debug_assertions) {
		debug
	} else {
		other
	}
}

/// Formats the [`RUN_TIMESTAMP`] with the [`Iso8601`] format.
fn fmt_run_timestamp() -> crate::Result<String> {
	RUN_TIMESTAMP
		.format(&Iso8601::DEFAULT)
		.wrap_err("unable to format run timestamp")
}

/// Initilizes different services of the application ([directories](dirs),
/// [logging](log), [panic handling](panic), etc.).
///
/// This function is intended to be called directly at the start of execution in
/// order to [RUN_TIMESTAMP] to be (lazily) evaluated right away.
#[tracing::instrument(skip_all)]
pub fn initialize_services(app_files: &AppFiles) -> crate::Result<()> {
	let _ = RUN_TIMESTAMP; // Immediately access and evaluate `RUN_TIMESTAMP`.
	oops::init_panic_handling()?;
	log::init_logging(app_files)?;
	app_files.log_project_dirs()?;
	Ok(())
}
