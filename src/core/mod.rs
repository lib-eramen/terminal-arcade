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

use crossterm::{
	cursor::{
		DisableBlinking,
		EnableBlinking,
		MoveTo,
	},
	event::{
		read,
		DisableBracketedPaste,
		DisableFocusChange,
		DisableMouseCapture,
		EnableBracketedPaste,
		Event,
		KeyCode, KeyEvent, KeyModifiers,
	},
	execute,
	style::{
		Attribute,
		Color,
		Print,
		SetAttribute,
		SetBackgroundColor,
	},
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
		Clear,
		ClearType,
	},
};
pub use outcomes::Outcome;
use tiny_gradient::{
	Gradient,
	GradientStr,
};

use self::terminal::get_terminal;
use crate::core::terminal::get_mut_terminal;

pub mod outcomes;
pub mod terminal;

/// The core struct to all inner workings in Terminal Arcade.
/// For now, this struct is a unit struct.
pub struct TerminalArcade;

/// The level of indentation to be used for printing.
pub static INDENT: &str = r#"        "#;

/// Terminal Arcade's ASCII banner.
pub const BANNER: &str = r#"
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
          /  /‾‾     /‾‾  /‾‾‾‾  /‾‾‾‾  /  / / / / / /‾/ / / /     / /\ \ \ \ \ \ \  ‾‾/
         /      /‾‾      /      /      /  / / / / / / / / / /     / /  \ \ \ \/ / / /‾‾
        /                             /  / / / / / / / / /  ‾‾‾/ / /‾‾‾‾\ \ \  / /  ‾‾/
        ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾   ‾‾  ‾‾  ‾‾  ‾‾  ‾‾‾‾‾‾  ‾‾      ‾‾  ‾‾  ‾‾‾‾‾
        "#; // These 8 spaces are added to keep up with the 8-space indentation of the
			// banner.

impl TerminalArcade {
	/// Sets global terminal rules.
	pub fn set_global_terminal_rules() -> Outcome<()> {
		enable_raw_mode()?;
		Ok(execute!(
			get_mut_terminal().backend_mut(),
			DisableBracketedPaste,
			DisableFocusChange,
			DisableMouseCapture,
			DisableBlinking,
			MoveTo(0, 0),
			SetBackgroundColor(Color::Black)
		)?)
	}

	/// Unsets the global terminal rules set in [`set_global_terminal_rules`].
	pub fn unset_global_terminal_rules() -> Outcome<()> {
		disable_raw_mode()?;
		Ok(execute!(
			get_mut_terminal().backend_mut(),
			EnableBracketedPaste,
			EnableBlinking,
		)?)
	}

	/// Returns a list of Print instructions for a stylized controls list.

	#[must_use]
	pub fn controls_list() -> Print<String> {
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
{INDENT}{}: {}uit...{reset} ({} works!)
"#,
			highlight("[1]"),
			highlight("[P]"),
			highlight("[2]"),
			highlight("[S]"),
			highlight("[0]"),
			highlight("[Q]"),
			highlight("[Ctrl-C]"),
		);
		Print(controls_list)
	}

	/// Prints a stylized version of the name "Terminal Arcade".
	pub fn print_introduction() -> Outcome<()> {
		let version = std::env::var("CARGO_PKG_VERSION")?;
		Ok(execute!(
			get_mut_terminal().backend_mut(),
			Clear(ClearType::All),
			SetAttribute(Attribute::Bold),
			Print(BANNER.to_string().gradient(Gradient::Fruit)),
			SetAttribute(Attribute::Reset),
			Print(format!("Terminal Arcade, v{version}").gradient(Gradient::Fruit)),
			Self::controls_list()
		)?)
	}

	/// The function to be called when Terminal Arcade starts up.
	pub fn startup() -> Outcome<()> {
		let _ = get_terminal(); // This call will initialize the global TERMINAL static variable.
		Self::print_introduction()?;
		Self::set_global_terminal_rules()?;
		Self::run()?;
		Ok(())
	}

	/// The function to ba called when Terminal Arcade is done starting and
	/// ready to start listening to events.
	pub fn run() -> Outcome<()> {
		loop {
			let event = read()?;
			if event == Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)) {
				Self::quit()?;
				break;
			}
		}
		Ok(())
	}

	/// The function to be called when Terminal Arcade is being quitted.
	pub fn quit() -> Outcome<()> {
		Self::unset_global_terminal_rules()?;
		Ok(())
	}
}
