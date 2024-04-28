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

use crate::core::Handler;

pub mod core;
pub mod games;
pub mod ui;

fn main() -> anyhow::Result<()> {
	let _ = color_eyre::install();
	Handler::new().startup()
}
