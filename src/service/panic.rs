//! Utilities for handling and setting up panic reports, using:
//! * [`color_eyre`]
//! * [`better_panic`] in debug builds
//! * [`human_panic`]

use std::panic::PanicInfo;

use color_eyre::config::PanicHook;
use tracing::{
	error,
	info,
};

lazy_static::lazy_static! {
	static ref REPO_URL: String = env!("CARGO_PKG_REPOSITORY").to_string();

	static ref PANIC_MSG: String = format!(
		"Terminal Arcade panicked! No, they does not need therapy and a bottle of Xanax, but they \
		 do need a bug report to {}! Please do they a favor and book it a trip to Bali. Thank \
		 you! 🎮 🐞",
		REPO_URL.clone()
	);
}

/// Panic hook for debugging, using [`better_panic`]'s backtrace.
#[cfg(debug_assertions)]
fn debug_panic_hook(panic_info: &PanicInfo) {
	better_panic::Settings::auto()
		.most_recent_first(false)
		.lineno_suffix(true)
		.verbosity(better_panic::Verbosity::Full)
		.create_panic_handler()(panic_info);
}

/// Panic hook for production, using [human_panic]'s reports.
#[cfg(not(debug_assertions))]
fn prod_panic_hook(panic_hook: &PanicHook, panic_info: &PanicInfo) {
	let meta = human_panic::Metadata::new(
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION"),
	);
	let file_path = human_panic::handle_dump(&meta, panic_info);

	human_panic::print_msg(file_path, &meta)
		.expect("human-panic: printing error message to console failed");
	eprintln!("{}", panic_hook.panic_report(panic_info));
}

/// Custom panic hook. Also resets the terminal to the original state in
/// addition to previous panic handling.
fn custom_panic_hook(panic_hook: &PanicHook, panic_info: &PanicInfo) {
	if let Err(err) = crate::tui::Tui::reset_terminal_rules() {
		error!(%err, "could not reset terminal rules");
	}
	let msg = format!("{}", panic_hook.panic_report(panic_info));
	error!("panic: {}", strip_ansi_escapes::strip_str(msg));

	#[cfg(debug_assertions)]
	debug_panic_hook(panic_info);
	#[cfg(not(debug_assertions))]
	prod_panic_hook(panic_hook, panic_info);

	eprintln!("{}", PANIC_MSG.clone());
	std::process::exit(libc::EXIT_FAILURE);
}

/// Initializes utilities for error/panic reporting.
/// Includes [`human_panic`], [`better_panic`] and [`color_eyre`].
pub fn init_panic_handling() -> crate::Result<()> {
	info!("initializing error & panic handling");
	std::env::set_var("RUST_BACKTRACE", match cfg!(debug_assertions) {
		true => "full",
		false => "1",
	});

	let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
		.panic_section(PANIC_MSG.clone())
		.capture_span_trace_by_default(true)
		.display_location_section(true)
		.try_into_hooks()?;

	eyre_hook.install()?;
	std::panic::set_hook(Box::new(move |panic_info| {
		custom_panic_hook(&panic_hook, panic_info);
	}));
	Ok(())
}
