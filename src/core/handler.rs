//! UI handler. Manages a hierarchy of screens and rendering them.

use std::{
	panic::{
		set_hook,
		take_hook,
	},
	path::{
		Path,
		PathBuf,
	},
	time::Duration,
};

use anyhow::bail;
use bool_toggle::Toggler;
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

use crate::{
	core::terminal::{
		get_terminal,
		initialize_terminal,
	},
	ui::{
		screens::{
			OpenStatus,
			ScreenAndState,
			ScreenKind,
			ScreenState,
			Screens,
		},
		util::clear_terminal,
		Screen,
		WelcomeScreen,
	},
};

/// Core struct to all inner workings in Terminal Arcade.
/// This struct mostly handles rendering that and managing screens.
#[must_use]
#[derive(Default)]
pub struct Handler {
	/// The screens stack in which Terminal Arcade manages windows.
	///
	/// The screen with the highest index in this hierarchy will be the only
	/// screen visible on the terminal.
	screen_stack: Vec<ScreenAndState>,
}

impl Handler {
	/// Creates a new handler object, registering this handler's terminal reset
	/// method to the panic hook ([`Self::unset_global_terminal_rules`]).
	pub fn new() -> Self {
		Self::set_panic_hook();
		Self::default()
	}

	/// Registers this handler's terminal reset method to the panic hook.
	/// ([`Self::unset_global_terminal_rules`])
	fn set_panic_hook() {
		let original_hook = take_hook();
		set_hook(Box::new(move |panic_info| {
			let _ = { Self::unset_global_terminal_rules() };
			original_hook(panic_info);
			println!("Sorry, something happened! 🫤\nIf you believe this was a bug, please send an issue to https://github.com/developer-ramen/terminal-arcade to get it squashed as soon as possible!");
		}));
	}

	/// Runs the event loop, also returning whether the loop should break.
	fn event_loop(&mut self, event: &Event) -> anyhow::Result<bool> {
		Ok(self.handle_terminal_event(event)? || self.handle_active_screen()?)
	}

	/// The function to be called when Terminal Arcade is done starting and
	/// ready to start listening to events.
	///
	/// Events, once handled by Terminal Arcade (for things like global
	/// shortcuts), are passed to the last screen (which is the only active
	/// screen anyways, see the struct documentation for more information).
	fn run(&mut self) -> anyhow::Result<()> {
		let sixty_fps_in_ms = 16;
		loop {
			self.draw_screen_ui()?;
			let poll_status = poll(Duration::from_millis(sixty_fps_in_ms))?;
			if poll_status && self.event_loop(&read()?)? {
				break;
			}
		}
		Ok(())
	}

	/// The function to be called when Terminal Arcade is being quitted.
	fn quit(&mut self) -> anyhow::Result<()> {
		while !self.screen_stack.is_empty() {
			self.close_active_screen()?;
		}
		Self::unset_global_terminal_rules()?;
		Ok(())
	}

	/// Gets the current active screen ***mutably***.
	fn get_mut_active_screen(&mut self) -> &mut ScreenAndState {
		self.screen_stack.last_mut().unwrap()
	}

	/// Gets screens that need to be drawn. This is determined by looking at the
	/// stack of screens and travelling top-down, looking until it encounters a
	/// parent screen (of [`ScreenKind::Normal`] variant) and collecting mutable
	/// references from the children screens that also need to be drawn.
	///
	/// The first element is the parent screen - what follows are the rest of
	/// the non-normal screens that all need to be rendered.
	fn get_drawn_screens(&mut self) -> Vec<&mut ScreenAndState> {
		let mut drawn_screens = Vec::new();
		for screen in self.screen_stack.iter_mut().rev() {
			let found_parent_screen = screen.state.kind == ScreenKind::Normal;
			drawn_screens.push(screen);
			if found_parent_screen {
				break;
			}
		}
		drawn_screens.reverse();
		drawn_screens
	}

	/// "Spawns" a screen. This method simply appends a
	/// [`ScreenAndState`] object to the tail end of the screen stack.
	fn spawn_screen(&mut self, screen: Screens) {
		self.screen_stack.push(ScreenAndState::new(screen));
	}

	/// Closes the active screen.
	/// Rhis function pops the screen from the screen hierarchy in
	/// Terminal Arcade, and calls its [`Screen::close`] function.
	fn close_active_screen(&mut self) -> anyhow::Result<()> {
		self.get_mut_active_screen().close()?;
		let _ = self.screen_stack.pop();
		Ok(())
	}

	/// Sets global terminal rules.
	fn set_global_terminal_rules() -> anyhow::Result<()> {
		enable_raw_mode()?;
		Ok(execute!(
			get_terminal().backend_mut(),
			DisableBracketedPaste,
			DisableFocusChange,
			DisableBlinking,
			EnterAlternateScreen,
			Hide,
			MoveTo(0, 0),
		)?)
	}

	/// Unsets the global terminal rules set in
	/// [`Self::set_global_terminal_rules`].
	fn unset_global_terminal_rules() -> anyhow::Result<()> {
		disable_raw_mode()?;
		let mut terminal = get_terminal();
		Ok(execute!(
			terminal.backend_mut(),
			EnableBracketedPaste,
			EnableFocusChange,
			EnableBlinking,
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

	/// Draws the UI. This function draws not only the topmost ("active") screen
	/// but also the parenting screens if the child(ren) screen is not of
	/// [`ScreenKind::Normal`] variant.
	fn draw_screen_ui(&mut self) -> anyhow::Result<()> {
		get_terminal().draw(|frame| {
			let drawn_screens = self.get_drawn_screens();
			let active_screen_index = drawn_screens.len() - 1;
			for (index, drawn_screen) in drawn_screens.into_iter().enumerate() {
				drawn_screen.screen.render(
					frame,
					&mut drawn_screen.state,
					index == active_screen_index,
				);
			}
		})?;
		Ok(())
	}

	/// Quits when the screen has no more screens to draw.
	/// Also returns if there are no screens, and by proxy, if the application
	/// has been quit.
	fn quit_when_no_screens(&mut self) -> anyhow::Result<bool> {
		Ok(if self.screen_stack.is_empty() {
			self.quit()?;
			true
		} else {
			false
		})
	}

	/// The function to be called when Terminal Arcade starts up.
	pub fn startup(&mut self) -> anyhow::Result<()> {
		initialize_terminal();
		Self::set_global_terminal_rules()?;
		self.spawn_screen(WelcomeScreen::default().into());
		self.run()?;
		Ok(())
	}

	/// Handles the information provided by the active screen,
	/// also returning if the event loop calling this function should quit.
	fn handle_active_screen(&mut self) -> anyhow::Result<bool> {
		if self.quit_when_no_screens()? {
			return Ok(true);
		}

		let active_screen = self.get_mut_active_screen();
		let created_screen = active_screen.state.screen_created.take();

		if active_screen.state.open_status == OpenStatus::Closed {
			self.close_active_screen()?;
		}
		if let Some(screen) = created_screen {
			self.spawn_screen(screen);
		}

		self.quit_when_no_screens()
	}

	/// Handles an event read from the terminal.
	/// also returning if the event loop calling this function should quit.
	fn handle_terminal_event(&mut self, event: &Event) -> anyhow::Result<bool> {
		match event {
			Event::Key(ref key) if Self::check_quit_controls(key) => {
				self.quit()?;
				return Ok(true);
			},
			Event::Resize(..) => {
				self.draw_screen_ui()?;
			},
			_ => {},
		}
		let screen = self.get_mut_active_screen();
		screen.screen.event(event, &mut screen.state)?;
		Ok(false)
	}
}
