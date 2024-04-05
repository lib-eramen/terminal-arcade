//! A module for containing the welcome screen in Terminal Arcade.

use std::cmp::max;

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
	},
	widgets::{
		Padding,
		Paragraph,
	},
	Frame,
};

use crate::{
	core::terminal::BackendType,
	ui::{
		components::{
			presets::{
				titled_ui_block,
				untitled_ui_block,
			},
			scroll_tracker::ScrollTracker,
			welcome::{
				controls::render_welcome_controls_block,
				footer::render_welcome_bottom_bar,
			},
		},
		screens::{
			config::ConfigScreen,
			game_select::GameSelectionScreen,
		},
		util::get_crate_version,
		Screen,
	},
};

/// Terminal Arcade's ASCII banner.
pub const BANNER: &str = r"/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾/‾‾/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\            
‾‾‾‾‾/  /‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾ ‾‾ ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\  \           
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
/                             /  / /‾/ / / / / / /  ‾‾‾/ / /‾‾‾‾\ \ \  / /  ‾‾/  
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾   ‾‾  ‾‾  ‾‾  ‾‾  ‾‾‾‾‾‾  ‾‾      ‾‾  ‾‾  ‾‾‾‾‾   ";

/// Screens that can be created by the welcome screen.
#[derive(PartialEq, Eq, Clone, Copy)]
enum ScreenCreated {
	/// The game selection screen.
	GameSelection,

	/// The settings screen.
	Settings,
}

/// The struct that welcomes the user to Terminal Arcade. To be presented every
/// time Terminal Arcade is started.
pub struct WelcomeScreen {
	/// Whether this screen is closing or not.
	closing: bool,

	/// The screen that this screen is spawning.
	screen_created: Option<ScreenCreated>,

	/// The scroll tracker for this screen.
	tracker: ScrollTracker,
}

impl Default for WelcomeScreen {
	fn default() -> Self {
		Self {
			closing: false,
			screen_created: None,
			tracker: ScrollTracker::new(3, None),
		}
	}
}

impl Screen for WelcomeScreen {
	fn render(&mut self, frame: &mut Frame<'_>) {
		let size = frame.size();
		frame.render_widget(titled_ui_block("Welcome to Terminal Arcade!"), size);
		let used_ui_height = 17 + 11 + 5 + 1;
		let empty_space_height =
			if size.height <= used_ui_height { 0 } else { size.height - used_ui_height };
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints(
				[
					Constraint::Max(16), // Banner's height + borders
					Constraint::Max(11), // Controls list block's height
					Constraint::Max(empty_space_height),
					Constraint::Max(5), // Bottom bar
				]
				.as_ref(),
			)
			.horizontal_margin(2)
			.split(size);
		let banner = Paragraph::new(BANNER).block(untitled_ui_block()).alignment(Alignment::Center);
		frame.render_widget(banner, chunks[0]);
		render_welcome_controls_block(chunks[1], frame, self.tracker.selected);
		render_welcome_bottom_bar(frame, chunks[3]);
	}

	fn event(&mut self, event: &Event) -> anyhow::Result<()> {
		if let Event::Key(key) = event {
			match key.code {
				KeyCode::Char(character) => {
					if key.modifiers != KeyModifiers::NONE {
						return Ok(());
					}
					self.handle_char_shortcut(character);
				},
				KeyCode::Up => self.tracker.scroll_up(),
				KeyCode::Down => self.tracker.scroll_down(),
				KeyCode::Enter => self.handle_enter_shortcut(),
				_ => {},
			}
		}
		Ok(())
	}

	fn screen_created(&mut self) -> Option<Box<dyn Screen>> {
		if let Some(screen) = self.screen_created {
			let screen_created = match screen {
				ScreenCreated::GameSelection => Self::create_game_selection_screen(),
				ScreenCreated::Settings => Self::create_settings_screen(),
			};
			self.screen_created = None;
			Some(screen_created)
		} else {
			None
		}
	}

	fn is_closing(&self) -> bool {
		self.closing
	}
}

impl WelcomeScreen {
	/// Marks the screen as closed.
	fn mark_closed(&mut self) {
		self.closing = true;
	}

	/// Sets the screen to be created from this welcome screen.
	fn set_screen_created(&mut self, screen_created: ScreenCreated) {
		self.screen_created = Some(screen_created);
	}

	// TODO: Game selection screen.
	/// Creates the game selection screen to switch to from this screen.
	fn create_game_selection_screen() -> Box<dyn Screen> {
		Box::<GameSelectionScreen>::default()
	}

	// TODO: Game selection screen.
	/// Creates the settings screen to switch to from this screen.
	fn create_settings_screen() -> Box<dyn Screen> {
		Box::<ConfigScreen>::default()
	}

	/// Handles the shortcut associated with the character inputted.
	fn handle_char_shortcut(&mut self, character: char) {
		match character {
			'1' | 'p' => self.set_screen_created(ScreenCreated::GameSelection),
			'2' | 'c' => self.set_screen_created(ScreenCreated::Settings),
			'0' | 'q' => self.mark_closed(),
			_ => {},
		}
	}

	/// Handles the ENTER shortcut, which executes the function that the UI
	/// selector is pointing at.
	fn handle_enter_shortcut(&mut self) {
		if let Some(index) = self.tracker.selected {
			match index {
				0 => self.set_screen_created(ScreenCreated::GameSelection),
				1 => self.set_screen_created(ScreenCreated::Settings),
				2 => self.mark_closed(),
				_ => panic!("Index not in predefined range (0..2) of welcome controls"),
			}
		}
	}
}
