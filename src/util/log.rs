//! Utilities for logging in Terminal Arcade, using [tracing].

use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
	layer::SubscriberExt,
	util::SubscriberInitExt,
	EnvFilter,
	Layer,
};

use crate::util::{
	dirs::get_data_dir,
	fmt_run_timestamp,
	PROJECT_NAME,
};

lazy_static::lazy_static! {
	pub static ref LOG_ENV_VAR: String =
		format!("{}_LOG_LEVEL", PROJECT_NAME.to_uppercase().clone());

	pub static ref LOG_FILE_NAME: String = format!(
		"{}-{}.log",
		PROJECT_NAME.clone(),
		fmt_run_timestamp()
	);
}

/// Initializes logging for Terminal Arcade.
///
/// The default [`EnvFilter`] behavior is to use the `RUST_LOG` environment
/// variable - when that is invalid, the [`LOG_ENV_VAR`] variable is used
/// instead. When even that is invalid, an error is returned.
pub fn init_logging() -> crate::Result<()> {
	let log_dir = get_data_dir().join("logs");
	std::fs::create_dir_all(log_dir.clone())?;
	let log_file_path = log_dir.join(LOG_FILE_NAME.clone());
	let log_file = std::fs::File::create(log_file_path)?;

	let env_filter =
		EnvFilter::builder().with_default_directive(LevelFilter::INFO.into());
	let env_filter = env_filter
		.try_from_env()
		.or_else(|_| env_filter.with_env_var(LOG_ENV_VAR.clone()).from_env())?;

	let file_subscriber = tracing_subscriber::fmt::layer()
		.with_ansi(false)
		.with_file(true)
		.with_line_number(true)
		.with_target(true)
		.with_thread_names(true)
		.with_writer(log_file)
		.with_filter(env_filter);
	tracing_subscriber::registry()
		.with(file_subscriber)
		.with(ErrorLayer::default())
		.try_init()?;

	Ok(())
}
