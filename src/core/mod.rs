//! # The `core` module
//!
//! This module contains some core functionality that all other functionalities
//! depend on.
//!
//! To get started, take a look at the [`TerminalArcade`] struct, the struct
//! that all mechanics in this crate is based on.

use std::{
	path::{
		Path,
		PathBuf,
	},
	time::Duration,
};

use crossterm::{
	cursor::{
		DisableBlinking,
		EnableBlinking,
		Hide,
		MoveTo,
		Show,
	},
	event::{
		poll,
		read,
		DisableBracketedPaste,
		DisableFocusChange,
		DisableMouseCapture,
		EnableBracketedPaste,
		EnableFocusChange,
		EnableMouseCapture,
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
use ratatui::{
	layout::{
		Constraint,
		Layout,
	},
	style::{
		Color,
		Style,
	},
};

use self::terminal::get_terminal;
use crate::{
	core::terminal::get_mut_terminal,
	ui::{
		util::clear_terminal,
		Screen,
		WelcomeScreen,
	},
};

pub mod outcomes;
pub mod terminal;

/// The directory where Terminal Arcade saves all of its data.
/// NOT TO BE USED DIRECTLY. This path does not include the home dir.
/// Use [`get_save_dir`] for this instead.
pub const SAVE_DIR: &str = ".local/share/terminal-arcade/";

/// Gets the save directory of Terminal Arcade.
/// Always use this function over the constant [`SAVE_DIR`].
#[must_use]
pub fn get_save_dir() -> PathBuf {
	home::home_dir().unwrap().as_path().to_owned().join(SAVE_DIR)
}

/// The core struct to all inner workings in Terminal Arcade.
/// Terminal Arcade has one encompassing UI construct - the root block. It is
/// instantiated first hand, and it nests all UI components and child screens.
/// This struct mostly handles rendering that and managing screens.
#[derive(Default)]
#[must_use]
pub struct Handler {
	/// The screens hierarchy in which Terminal Arcade manages windows.
	///
	/// The hierarchy is linear - there is always the root window which is the
	/// startup screen or setup screen (depending on if the setup has been run
	/// yet), and it moves on to game screens, setting screens, etc. Simple,
	/// one-time popups can also take advantage of this structure.
	///
	/// According to how this should be implemented, the screen with the highest
	/// index in this hierarchy will be the only screen painted - in particular,
	/// it will be the only screen visible on the terminal, aside from its
	/// nesting root block that is globally visible.
	screens: Vec<Box<dyn Screen>>,
}

impl Handler {
	/// Constructs a new Terminal Arcade object.
	pub fn new() -> Self {
		Self { screens: vec![] }
	}

	/// Gets the current active screen.
	fn get_active_screen(&self) -> &dyn Screen {
		self.screens.last().unwrap().as_ref()
	}

	/// Gets the current active screen ***mutably***.
	fn get_mut_active_screen(&mut self) -> &mut dyn Screen {
		self.screens.last_mut().unwrap().as_mut()
	}

	/// Spawns a screen as active.
	pub fn spawn_screen(&mut self, screen: Box<dyn Screen>) {
		self.screens.push(screen);
	}

	/// Closes the active screen.
	/// In detail, this function pops the screen from the screen hierarchy in
	/// Terminal Arcade, and calls its [`Screen::close`] function.
	pub fn close_screen(&mut self) -> Outcome<()> {
		self.get_mut_active_screen().close()?;
		let _ = self.screens.pop();
		Ok(())
	}

	/// Sets global terminal rules.
	fn set_global_terminal_rules() -> Outcome<()> {
		enable_raw_mode()?;
		Ok(execute!(
			get_mut_terminal().backend_mut(),
			DisableBracketedPaste,
			DisableFocusChange,
			DisableBlinking,
			EnterAlternateScreen,
			Hide,
			MoveTo(0, 0),
		)?)
	}

	/// Unsets the global terminal rules set in [`set_global_terminal_rules`].
	fn unset_global_terminal_rules() -> Outcome<()> {
		disable_raw_mode()?;
		Ok(execute!(
			get_mut_terminal().backend_mut(),
			EnableBracketedPaste,
			EnableFocusChange,
			EnableBlinking,
			DisableMouseCapture,
			LeaveAlternateScreen,
			Show,
		)?)
	}

	/// Checks for whether a key event matches the quit controls.
	#[must_use]
	fn check_quit_controls(key: &KeyEvent) -> bool {
		let quit_controls = [
			KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL),
			KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
			KeyEvent::new(KeyCode::F(4), KeyModifiers::ALT),
		];
		quit_controls.contains(key)
	}

	/// Draws the UI of the active screen.
	fn draw_active_screen_ui(&mut self) -> Outcome<()> {
		get_mut_terminal().draw(|frame| self.get_active_screen().draw_ui(frame))?;
		Ok(())
	}

	/// Quits when the screen has no more screens to draw.
	/// Also returns if there are no screens, and by proxy, if the application
	/// has been quit.
	fn quit_when_no_screens(&mut self) -> Outcome<bool> {
		Ok(if self.screens.is_empty() {
			self.quit()?;
			true
		} else {
			false
		})
	}

	/// The function to be called when Terminal Arcade starts up.
	pub fn startup(&mut self) -> Outcome<()> {
		let _ = get_terminal(); // This call will initialize the global TERMINAL static variable.
		Self::set_global_terminal_rules()?;
		self.spawn_screen(Box::<WelcomeScreen>::default());
		self.run()?;
		Ok(())
	}

	/// Handles the information provided by the active screen,
	/// also returning if the event loop calling this function should quit.
	fn handle_active_screen(&mut self) -> Outcome<bool> {
		if self.quit_when_no_screens()? {
			return Ok(true);
		}
		let active_screen = self.get_mut_active_screen();
		let created_screen: Option<Box<dyn Screen>> = active_screen.screen_created();
		if active_screen.is_closing() {
			self.close_screen()?;
		}
		if let Some(screen) = created_screen {
			self.spawn_screen(screen);
		}
		self.quit_when_no_screens()
	}

	/// Handles an event read from the terminal.
	/// also returning if the event loop calling this function should quit.
	fn handle_terminal_event(&mut self, event: &Event) -> Outcome<bool> {
		match event {
			Event::Key(ref key) if Self::check_quit_controls(key) => {
				self.quit()?;
				return Ok(true);
			},
			Event::Key(ref key) if key.code == KeyCode::Esc => {
				self.close_screen()?;
				if self.quit_when_no_screens()? {
					return Ok(true);
				}
			},
			Event::Resize(..) => {
				self.draw_active_screen_ui()?;
			},
			_ => {},
		}
		self.get_mut_active_screen().event(event)?;
		Ok(false)
	}

	/// Runs the event loop, also returning whether the loop should break.
	fn event_loop(&mut self, event: &Event) -> Outcome<bool> {
		Ok(self.handle_terminal_event(event)? || self.handle_active_screen()?)
	}

	/// The function to be called when Terminal Arcade is done starting and
	/// ready to start listening to events.
	///
	/// Events, once handled by Terminal Arcade (for things like global
	/// shortcuts), are passed to the last screen (which is the only active
	/// screen anyways, see the struct documentation for more information).
	fn run(&mut self) -> Outcome<()> {
		let sixty_fps_in_ms = 16;
		loop {
			self.draw_active_screen_ui()?;
			let poll_status = poll(Duration::from_millis(sixty_fps_in_ms))?; // 60 FPS
			if poll_status && self.event_loop(&read()?)? {
				break;
			}
		}
		Ok(())
	}

	/// The function to be called when Terminal Arcade is being quitted.
	fn quit(&mut self) -> Outcome<()> {
		while !self.screens.is_empty() {
			self.close_screen()?;
		}
		Self::unset_global_terminal_rules()?;
		Ok(())
	}
}
