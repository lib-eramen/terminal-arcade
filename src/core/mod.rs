//! # The `core` module
//!
//! This module contains all of the inner workings and mechanisms for Terminal
//! Arcade - including its UI, text, game loading, and custom game engine.
//!
//! Okay, "custom game engine" sounds a bit overblown. It's just an event
//! handling trait.
//!
//! To get started, take a look at the [`TerminalArcade`] struct, the struct
//! that all mechanics in this crate is based on.

/// The core struct to all inner workings in Terminal Arcade.
/// For now, this struct is a unit struct.
pub struct TerminalArcade;

impl TerminalArcade {
	/// The function to be called when Terminal Arcade starts up.
	pub fn startup() {
		todo!()
	}
}
