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
use strum::{
	Display,
	EnumString,
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
			scrollable_list::{
				ListItem,
				ScrollableList,
			},
			welcome::footer::render_welcome_bottom_bar,
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
	GameSearch,
	Settings,
}

/// Control options available at the welcome screen.
#[derive(Clone, Copy, PartialEq, Eq, Display)]
enum ControlOptions {
	SearchGames,
	ViewSettings,
	QuitApplication,
}

/// The struct that welcomes the user to Terminal Arcade. To be presented every
/// time Terminal Arcade is started.
pub struct WelcomeScreen {
	/// Whether this screen is closing or not.
	closing: bool,

	/// Screen that this screen is spawning.
	screen_created: Option<ScreenCreated>,

	/// Scrollable list widget for options.
	controls_list: ScrollableList<ControlOptions>,
}

impl Default for WelcomeScreen {
	fn default() -> Self {
		let controls_list = ScrollableList::new(
			vec![
				ListItem::new(
					None,
					ControlOptions::SearchGames,
					Some("🎮 Hop into a game and play!".to_string()),
				),
				ListItem::new(
					None,
					ControlOptions::ViewSettings,
					Some("🗜️ View your settings...".to_string()),
				),
				ListItem::new(
					None,
					ControlOptions::QuitApplication,
					Some("🛑 Quit the application...".to_string()),
				),
			],
			None,
			1,
			Direction::Vertical,
			Alignment::Center,
			None,
		);
		Self {
			closing: false,
			screen_created: None,
			controls_list,
		}
	}
}

impl Screen for WelcomeScreen {
	fn render(&mut self, frame: &mut Frame<'_>) {
		let size = frame.size();
		frame.render_widget(titled_ui_block("Welcome to Terminal Arcade!"), size);
		let used_ui_height = 16 + 11 + 5 + 6;
		let empty_space_height =
			if size.height <= used_ui_height { 0 } else { size.height - used_ui_height };
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints([
				Constraint::Max(16), // Banner's height + borders
				Constraint::Max(11), // Controls list block's height
				Constraint::Min(empty_space_height),
				Constraint::Max(6), // Bottom bar
			])
			.horizontal_margin(2)
			.split(size);
		let banner = Paragraph::new(BANNER).block(untitled_ui_block()).alignment(Alignment::Center);
		frame.render_widget(banner, chunks[0]);
		self.controls_list.render(frame, chunks[1]);
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
				KeyCode::Up => self.controls_list.scroll_tracker.scroll_forward(),
				KeyCode::Down => self.controls_list.scroll_tracker.scroll_backward(),
				KeyCode::Enter => self.handle_enter_shortcut(),
				_ => {},
			}
		}
		Ok(())
	}

	fn screen_created(&mut self) -> Option<Box<dyn Screen>> {
		if let Some(screen) = self.screen_created {
			let screen_created = match screen {
				ScreenCreated::GameSearch => Self::create_game_selection_screen(),
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

	/// Creates the game selection screen to switch to from this screen.
	fn create_game_selection_screen() -> Box<dyn Screen> {
		Box::<GameSelectionScreen>::default()
	}

	/// Creates the settings screen to switch to from this screen.
	fn create_settings_screen() -> Box<dyn Screen> {
		Box::<ConfigScreen>::default()
	}

	/// Handles the shortcut associated with the character inputted.
	fn handle_char_shortcut(&mut self, character: char) {
		match character {
			'1' | 'p' => self.set_screen_created(ScreenCreated::GameSearch),
			'2' | 'c' => self.set_screen_created(ScreenCreated::Settings),
			'0' | 'q' => self.mark_closed(),
			_ => {},
		}
	}

	/// Handles the ENTER shortcut, which executes the function that the UI
	/// selector is pointing at.
	fn handle_enter_shortcut(&mut self) {
		if let Some((_, item)) = self.controls_list.get_selected() {
			match item.data {
				ControlOptions::SearchGames => self.set_screen_created(ScreenCreated::GameSearch),
				ControlOptions::ViewSettings => self.set_screen_created(ScreenCreated::Settings),
				ControlOptions::QuitApplication => self.mark_closed(),
			}
		}
	}
}
