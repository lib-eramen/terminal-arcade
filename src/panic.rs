//! Utilities for handling and setting up panic reports.

use std::{
	env,
	panic::PanicHookInfo,
};

use color_eyre::{
	config::PanicHook,
	eyre::Result,
};
use lazy_static::lazy_static;

lazy_static! {
	static ref REPO_URL: String = env!("CARGO_PKG_REPOSITORY").to_string();
	static ref PANIC_MSG: String = format!(
		"Terminal Arcade panicked! No, it does not need therapy and a bottle of Xanax, but it \
		 does need a bug report to {}! Please do it a favor and book it a trip to Bali. Thank \
		 you! 🎮 🐞",
		REPO_URL.clone()
	);
}

/// Panic hook for debugging, using [better_panic]'s stacktrace.
fn debug_panic_hook(panic_info: &PanicHookInfo) {
	better_panic::Settings::auto()
		.most_recent_first(false)
		.lineno_suffix(true)
		.verbosity(better_panic::Verbosity::Full)
		.create_panic_handler()(panic_info);
}

/// Panic hook for production, using [human_panic]'s reports.
fn prod_panic_hook(panic_hook: &PanicHook, panic_info: &PanicHookInfo) {
	let meta = human_panic::Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
	let file_path = human_panic::handle_dump(&meta, panic_info);
	human_panic::print_msg(file_path, &meta)
		.expect("human-panic: printing error message to console failed");
	eprintln!("{}", panic_hook.panic_report(panic_info));
}

/// Custom panic hook that undoes the side effects of [Tui::set_terminal_rules]
/// and resets the terminal to the original state in addition to previous panic
/// handling.
fn custom_panic_hook(panic_hook: &PanicHook, panic_info: &PanicHookInfo) {
	// TODO: Reset terminal rules here
	let msg = format!("{}", panic_hook.panic_report(panic_info));
	log::error!("Error: {}", strip_ansi_escapes::strip_str(msg));
	if cfg!(debug_assertions) {
		debug_panic_hook(panic_info);
	} else {
		prod_panic_hook(panic_hook, panic_info);
	}
	std::process::exit(libc::EXIT_FAILURE)
}

/// Initializes utilities for logging and error/panic reporting.
/// Includes [human_panic], [better_panic] and [color_eyre].
pub fn init_panic_handling() -> Result<()> {
	let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
		.panic_section(PANIC_MSG.clone())
		.capture_span_trace_by_default(false)
		.display_location_section(false)
		.display_env_section(false)
		.into_hooks();
	eyre_hook.install()?;
	std::panic::set_hook(Box::new(move |panic_info| {
		custom_panic_hook(&panic_hook, panic_info)
	}));
	Ok(())
}
