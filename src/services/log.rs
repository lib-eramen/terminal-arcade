//! Utilities for tracing in Terminal Arcade, using [`tracing`].
//! It's named `log` because, well, [`tracing`].

use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
	layer::SubscriberExt,
	util::SubscriberInitExt,
	EnvFilter,
	Layer,
};

use crate::services::{
	debug_either,
	dirs::AppDirs,
	fmt_run_timestamp,
	PROJECT_NAME,
};

lazy_static::lazy_static! {
	pub static ref LOG_ENV_VAR: String =
		format!("{}_LOG_LEVEL", PROJECT_NAME.to_uppercase().clone());
}

fn get_log_file_name() -> crate::Result<String> {
	Ok(format!(
		"{}-{}.log",
		PROJECT_NAME.clone(),
		fmt_run_timestamp()?
	))
}

/// Initializes logging for Terminal Arcade.
///
/// The default [`EnvFilter`] behavior is to use the `RUST_LOG` environment
/// variable - when that is invalid, the [`LOG_ENV_VAR`] variable is used
/// instead. When even that is invalid, an error is returned.
pub fn init_logging(app_dirs: &AppDirs) -> crate::Result<()> {
	tracing::info!("initializing logging");
	let (log_dir, _) = app_dirs.get_data_dir("log", Some("logs".into()))?;

	std::fs::create_dir_all(log_dir.clone())?;
	let log_file_path = log_dir.join(get_log_file_name()?);
	let log_file = std::fs::File::create(log_file_path)?;

	let env_filter = EnvFilter::builder().with_default_directive(
		debug_either(LevelFilter::TRACE, LevelFilter::INFO).into(),
	);
	let env_filter = env_filter
		.try_from_env()
		.or_else(|_| env_filter.with_env_var(LOG_ENV_VAR.clone()).from_env())?;
	let file_subscriber = tracing_subscriber::fmt::layer()
		.with_ansi(false)
		.with_writer(log_file)
		.with_filter(env_filter);

	tracing_subscriber::registry()
		.with(file_subscriber)
		.with(ErrorLayer::default())
		.try_init()?;
	Ok(())
}
