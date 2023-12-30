//! # Terminal Arcade
//!
//! Terminal Arcade is an arcade machine
//! replica-concept-reinvention-do-it-myself thingymajig of the arcade
//! machine! The preceding sentence should have already given a solid indication
//! of the quality of this crate.
//!
//! This crate contains an interface for extending and building more games, as
//! well as a (hopefully) lot other pre-built games as well.

#![deny(unused_must_use, unused_imports, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, missing_docs)]
#![allow(
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::module_name_repetitions,
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	unused_imports
)]

use crate::core::{
	Handler,
	Outcome,
};

pub mod core;
pub mod game;
pub mod ui;

/// Entry point of a program! What, you expected a `main` function to be
/// documented well?
#[must_use = "I have no idea why rust-analyzer made me put this here"]
fn main() -> Outcome<()> {
	Handler::new().startup()
}
