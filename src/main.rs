//! # Terminal Arcade
//!
//! Terminal-based arcade-style games for when you're bored out of your mind.
//!
//! Expect this to be a work-in-progress always! New games and features and
//! to-be-debugged spaghetti code guaranteed.

use color_eyre::eyre::Result;

use crate::{
	log::init_logging,
	panic::init_panic_handling,
};

pub mod log;
pub mod panic;

fn main() -> Result<()> {
	init_panic_handling()?;
	init_logging()?;
	Ok(())
}
