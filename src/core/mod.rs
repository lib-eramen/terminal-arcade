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
	},
	terminal::{
		Clear,
		ClearType,
	},
};
pub use outcomes::Outcome;
use tiny_gradient::{
	Gradient,
	GradientStr,
};

pub mod outcomes;

/// The core struct to all inner workings in Terminal Arcade.
/// For now, this struct is a unit struct.
pub struct TerminalArcade;

/// The level of indentation to be used for printing.
pub static INDENT: &str = r#"        "#;

/// Terminal Arcade's ASCII banner.
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
	/// Prints a stylized controls list.
	pub fn stylized_controls_list() -> Print<String> {
		/// Highlights text as bold and rainbow.
		/// Note that you might need to reset the text after applying the bold
		/// attribute.
		fn highlight(text: &str) -> String {
			format!("{}{}", Attribute::Bold, text.gradient(Gradient::Fruit))
		}
		let reset = Attribute::Reset;
		let controls_list = format!(
			r#"
{INDENT}{}: Choose a game to {}lay!{reset}
{INDENT}{}: View your {}ettings!{reset}
{INDENT}{}: {}uit...{reset}
"#,
			highlight("[1]"),
			highlight("[P]"),
			highlight("[2]"),
			highlight("[S]"),
			highlight("[0]"),
			highlight("[Q]"),
		);
		Print(controls_list)
	}

	/// Prints a stylized version of the name "Terminal Arcade".
	pub fn print_stylized_title() -> Outcome<()> {
		let version = std::env::var("CARGO_PKG_VERSION")?;
		Ok(execute!(
			stdout(),
			Clear(ClearType::All),
			SetAttribute(Attribute::Bold),
			Print(BANNER.to_string().gradient(Gradient::Fruit)),
			SetAttribute(Attribute::Reset),
			Print(format!("Terminal Arcade, v{version}").gradient(Gradient::Fruit)),
			Self::stylized_controls_list()
		)?)
	}

	/// The function to be called when Terminal Arcade starts up.
	pub fn startup() -> Outcome<()> {
		Self::print_stylized_title()
	}
}
