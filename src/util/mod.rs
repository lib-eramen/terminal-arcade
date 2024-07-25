//! Utilities specific to inner workings for Terminal Arcade, such as
//! [logging](log), [error and panic handling](panic), [directories](dirs), etc.

use color_eyre::eyre::eyre;
use time::{
	format_description::well_known::Iso8601,
	OffsetDateTime,
};
use tracing::{
	info,
	instrument,
	trace,
};

pub mod dirs;
pub mod log;
pub mod panic;

lazy_static::lazy_static! {
	/// This crate/app/project's name in lowercase.
	pub static ref PROJECT_NAME: String =
		env!("CARGO_PKG_NAME").to_lowercase().to_string();

	/// The timestamp of when Terminal Arcade was run.
	pub static ref RUN_TIMESTAMP: OffsetDateTime = OffsetDateTime::now_utc();
}

/// Formats the [`RUN_TIMESTAMP`] with the [`Iso8601`] format.
fn fmt_run_timestamp() -> crate::Result<String> {
	Ok(RUN_TIMESTAMP
		.format(&Iso8601::DEFAULT)
		.map_err(|err| eyre!("unable to format run timestamp: {err}"))?)
}

/// Logs the current running mode.
fn log_current_running_mode() {
	info!(
		"current running mode: {}",
		if cfg!(debug_assertions) {
			"debug"
		} else {
			"release"
		}
	);
}

/// Initilizes different utilities of the application ([directories](dirs),
/// [logging](log), [panic handling](panic), etc.).
///
/// This function is intended to be called directly at the start of execution in
/// order to [RUN_TIMESTAMP] to be (lazily) evaluated right away.
#[instrument]
pub fn initialize_utils() -> crate::Result<()> {
	let _ = RUN_TIMESTAMP; // Immediately access and evaluate `RUN_TIMESTAMP`.
	log::init_logging()?;
	log_current_running_mode();

	let fmted_timestamp = fmt_run_timestamp()
		.map_err(|err| eyre!("unable to fmt run timestamp: {err}"))?;
	trace!("initialized run timestamp: {fmted_timestamp}");

	panic::init_panic_handling()?;
	dirs::init_project_dirs()?;
	Ok(())
}
