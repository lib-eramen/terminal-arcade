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

use std::io::stdout;

use crossterm::{
	execute,
	style::{
		Attribute,
		Print,
		SetAttribute,
	}, terminal::{Clear, ClearType},
};
use tiny_gradient::{
	Gradient,
	GradientStr,
};

pub use outcomes::Outcome;

pub mod outcomes;

/// The core struct to all inner workings in Terminal Arcade.
/// For now, this struct is a unit struct.
pub struct TerminalArcade;

pub static INDENT: &str = r#"        "#;

pub const BANNER: &'static str = r#"
        /‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾////‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\
        ‾‾‾‾‾/  /‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾////‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\  \
            /  /  /‾‾‾‾‾/  /‾‾‾‾‾‾‾/  /‾‾‾‾‾‾‾‾‾/  /‾‾/  /‾‾‾‾‾‾/  /‾‾‾‾\  \  \
           /  /  /  /‾‾‾  /  /‾/  /  / /‾/ /‾/ /  /  /  /  /‾/ /  /  /\  \  \  \
          /  /  /  ‾‾‾/  /  / /  /  / / / / / /  /  /  /  / / /  /   ‾‾   \  \  \
         /  /  /  /‾‾‾  /  /  \  \  \ \ \ \ \ \  \  \  \  \ \ \  \  \‾‾‾\  \  \  \
        /  /  /  ‾‾‾/  /  /    \  \  \ \ \ \ \ \  \  \  \  \ \ \  \  \   \  \  \  ‾‾‾‾‾\
        ‾‾‾   ‾‾‾‾‾‾   ‾‾‾      ‾‾‾   ‾‾  ‾‾  ‾‾   ‾‾‾   ‾‾‾  ‾‾   ‾‾‾    ‾‾‾   ‾‾‾‾‾‾‾‾
            /‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾/  /‾‾\ \‾‾‾‾‾\ \‾‾‾‾‾‾‾‾/ /\ \‾‾‾‾‾\ \‾‾‾‾‾‾\
           /      /‾‾                    /  / /\ \ \ \‾\ \ \ \‾‾‾‾  /  \ \ \‾\ \ \  \‾‾‾
          /  /‾‾     /‾‾  /‾‾‾‾  /‾‾‾‾  /  / / / / / /‾/ / / / /   / /\ \ \ \ \ \ \  ‾‾/
         /      /‾‾      /      /      /  / / / / / / / / / /     / /  \ \ \ \/ / / /‾‾
        /                             /  / / / / / / / / /  ‾‾‾/ / /‾‾‾‾\ \ \  / /  ‾‾/
        ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾   ‾‾  ‾‾  ‾‾  ‾‾  ‾‾‾‾‾‾  ‾‾      ‾‾  ‾‾  ‾‾‾‾‾
        "#; // These 8 spaces are added to keep up with the 8-space indentation of the
			// banner.

impl TerminalArcade {
	/// Prints a stylized version of the name "Terminal Arcade".
	pub fn print_stylized_title() -> Outcome<()> {
		let version = std::env::var("CARGO_PKG_VERSION")?;
		execute!(
			stdout(),
			Clear(ClearType::All),
			SetAttribute(Attribute::Bold),
			Print(BANNER.to_string().gradient(Gradient::Rainbow)),
			SetAttribute(Attribute::Reset),
			Print(format!("Terminal Arcade, v{version}").gradient(Gradient::Rainbow)),
		)?;
		Ok(())
	}

	/// The function to be called when Terminal Arcade starts up.
	pub fn startup() -> Outcome<()> {
		Self::print_stylized_title()
	}
}
