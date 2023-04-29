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
		Hide,
		MoveTo,
		Show,
	},
	event::{
		read,
		DisableBracketedPaste,
		DisableFocusChange,
		DisableMouseCapture,
		EnableBracketedPaste,
		Event,
		KeyCode,
		KeyEvent,
		KeyModifiers,
	},
	execute,
	terminal::{
		disable_raw_mode,
		enable_raw_mode,
		EnterAlternateScreen,
		LeaveAlternateScreen,
	},
};
pub use outcomes::Outcome;

use self::terminal::get_terminal;
use crate::{
	core::terminal::get_mut_terminal,
	screens::{
		util::clear_terminal,
		Screen,
		WelcomeScreen,
	},
};

pub mod outcomes;
pub mod terminal;

/// The core struct to all inner workings in Terminal Arcade.
#[derive(Default)]
#[must_use]
pub struct TerminalArcade {
	/// The screens hierarchy in which Terminal Arcade manages windows.
	///
	/// The hierarchy is linear - there is always the root window which is the
	/// startup screen or setup screen (depending on if the setup has been run
	/// yet), and it moves on to game screens, setting screens, etc. Simple,
	/// one-time popups can also take advantage of this structure.
	///
	/// According to how this should be implemented, the window with the highest
	/// index in this hierarchy will be painted - in particular, it will be the
	/// only screen visible on the terminal.
	screens: Vec<Box<dyn Screen>>,
}

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
	/// Constructs a new Terminal Arcade object.
	pub fn new() -> Self {
		Self { screens: vec![] }
	}

	/// Gets the current active screen.
	pub fn get_active_screen(&self) -> &dyn Screen {
		self.screens.last().unwrap().as_ref()
	}

	/// Gets the current active screen ***mutably***.
	pub fn get_mut_active_screen(&mut self) -> &mut dyn Screen {
		self.screens.last_mut().unwrap().as_mut()
	}

	/// Spawns an active screen.
	/// In detail, this function pushes the screen to the top of the screen
	/// hierarchy in Terminal Arcade, and calls its [`Screen::spawn`] function.
	pub fn spawn_screen(&mut self, mut screen: Box<dyn Screen>) -> Outcome<()> {
		clear_terminal()?;
		screen.on_spawn()?;
		self.screens.push(screen);
		Ok(())
	}

	/// Closes the active screen.
	/// In detail, this function pops the screen from the screen hierarchy in
	/// Terminal Arcade, and calls its [`Screen::close`] function.
	pub fn close_screen(&mut self) -> Outcome<()> {
		clear_terminal()?;
		self.get_mut_active_screen().on_close()?;
		let _ = self.screens.pop();
		Ok(())
	}

	/// Sets global terminal rules.
	pub fn set_global_terminal_rules() -> Outcome<()> {
		enable_raw_mode()?;
		Ok(execute!(
			get_mut_terminal().backend_mut(),
			DisableBracketedPaste,
			DisableFocusChange,
			DisableMouseCapture,
			DisableBlinking,
			EnterAlternateScreen,
			Hide,
			MoveTo(0, 0),
		)?)
	}

	/// Unsets the global terminal rules set in [`set_global_terminal_rules`].
	pub fn unset_global_terminal_rules() -> Outcome<()> {
		disable_raw_mode()?;
		Ok(execute!(
			get_mut_terminal().backend_mut(),
			EnableBracketedPaste,
			EnableBlinking,
			LeaveAlternateScreen,
			Show,
		)?)
	}

	/// Checks for whether a key event matches the quit controls.
	#[must_use]
	pub fn check_quit_controls(key_event: &KeyEvent) -> bool {
		let quit_controls = vec![
			KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL),
			KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
			KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
			KeyEvent::new(KeyCode::F(4), KeyModifiers::ALT),
		];
		quit_controls.contains(key_event)
	}

	/// The function to be called when Terminal Arcade starts up.
	pub fn startup(&mut self) -> Outcome<()> {
		let _ = get_terminal(); // This call will initialize the global TERMINAL static variable.
		Self::set_global_terminal_rules()?;
		Self::spawn_screen(self, Box::new(WelcomeScreen))?;
		Self::run(self)?;
		Ok(())
	}

	/// The function to be called when Terminal Arcade is done starting and
	/// ready to start listening to events.
	///
	/// Events, once handled by Terminal Arcade (for things like global
	/// shortcuts), are passed to the last screen (which is the only active
	/// screen anyways, see the struct documentation for more information).
	pub fn run(&mut self) -> Outcome<()> {
		loop {
			let event = read()?;
			if let Event::Key(ref key_event) = event {
				if Self::check_quit_controls(key_event) {
					Self::quit(self)?;
					break;
				}
			}
			self.get_mut_active_screen().on_event(&event)?;
		}
		Ok(())
	}

	/// The function to be called when Terminal Arcade is being quitted.
	pub fn quit(&mut self) -> Outcome<()> {
		while !self.screens.is_empty() {
			self.close_screen()?;
		}
		Self::unset_global_terminal_rules()?;
		Ok(())
	}
}
