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
		SetAttributes,
	},
};

pub mod outcomes;

/// The core struct to all inner workings in Terminal Arcade.
/// For now, this struct is a unit struct.
pub struct TerminalArcade;

impl TerminalArcade {
	/// Prints a stylized version of the name "Terminal Arcade".
	pub fn print_stylized_title() -> Outcome<()> {
		let version = std::env::var("CARGO_PKG_VERSION")?;
		let title = r#"
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
        "#;
		execute!(
			stdout(),
			SetAttribute(Attribute::Bold),
			Print(title.to_string()),
			SetAttributes(
				[
					Attribute::Reset,
					Attribute::Underlined,
					Attribute::Bold,
					Attribute::Italic
				]
				.as_slice()
				.into()
			),
			Print(format!(
				"Terminal Arcade{}{}{}",
				Attribute::Reset,
				format!(", v{version}"),
				SetAttribute(Attribute::Reset)
			)),
		)?;
		Ok(())
	}

	/// The function to be called when Terminal Arcade starts up.
	pub fn startup() -> Outcome<()> {
		Self::print_stylized_title()
	}
}

pub use outcomes::Outcome;
