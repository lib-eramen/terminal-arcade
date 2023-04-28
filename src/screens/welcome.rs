//! A module for containing the welcome screen in Terminal Arcade.

use crossterm::{
	event::Event,
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
use tiny_gradient::{
	Gradient,
	GradientStr,
};

use super::{
	highlight,
	Screen,
	INDENT,
};
use crate::{
	core::{
		terminal::get_mut_terminal,
		Outcome,
	},
	disable_raw_mode,
};

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

/// The struct that welcomes the user to Terminal Arcade. To be presented every
/// time Terminal Arcade is started.
pub struct WelcomeScreen;

impl Screen for WelcomeScreen {
	fn on_spawn(&mut self) -> Outcome<()> {
		disable_raw_mode! {
			Self::print_introduction()?
		};
		Ok(())
	}

	/// TODO: Implement the event handler for the welcome screen
	fn on_event(&mut self, _event: &Event) -> Outcome<()> {
		Ok(())
	}

	fn on_close(&mut self) -> Outcome<()> {
		Ok(())
	}
}

impl WelcomeScreen {
	/// Returns a [Print] instruction for a stylized controls list.
	#[must_use]
	pub fn controls_list() -> Print<String> {
		let reset = Attribute::Reset;
		let controls_list = format!(
			r#"
{INDENT}{}: Choose a game to {}! ({}){reset}
{INDENT}{}: View your {} ({})!{reset}
{INDENT}{}: {}uit...{reset} ({} and {} also work!)
"#,
			highlight("[1]"),
			highlight("play"),
			highlight("[Ctrl-P]"),
			highlight("[2]"),
			highlight("settings"),
			highlight("[Ctrl-,]"),
			highlight("[0]"),
			highlight("[Q]"),
			highlight("[Ctrl-C]"),
			highlight("[Esc]")
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
}
