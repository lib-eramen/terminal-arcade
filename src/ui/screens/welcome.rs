//! A module for containing the welcome screen in Terminal Arcade.

use std::cmp::max;

use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
	layout::{Alignment, Constraint, Direction, Layout},
	widgets::{Padding, Paragraph},
	Frame,
};
use strum::{Display, EnumString};

use crate::{
	core::terminal::BackendType,
	ui::{
		components::{
			presets::{titled_ui_block, untitled_ui_block},
			welcome::footer::render_welcome_bottom_bar,
		},
		screens::{
			config::ConfigScreen, game_select::GameSearchScreen, OpenStatus, ScreenAndState,
			ScreenKind, ScreenState, Screens,
		},
		util::get_crate_version,
		widgets::scrollable_list::{ListItem, ScrollableList},
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

/// Control options available at the welcome screen.
#[derive(Clone, Copy, PartialEq, Eq, Display)]
enum ControlOptions {
	SearchGames,
	ViewConfigs,
	QuitApplication,
}

/// The struct that welcomes the user to Terminal Arcade. To be presented every
/// time Terminal Arcade is started.
#[derive(Clone)]
pub struct WelcomeScreen {
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
					ControlOptions::ViewConfigs,
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
			Some((1, 3)),
			None,
		);
		Self { controls_list }
	}
}

impl Screen for WelcomeScreen {
	fn initial_state(&self) -> ScreenState {
		ScreenState::new("Terminal Arcade", ScreenKind::Normal, None)
	}

	fn handle_event(&mut self, event: &Event, state: &mut ScreenState) -> anyhow::Result<()> {
		if let Event::Key(key) = event {
			match key.code {
				KeyCode::Up => self.controls_list.scroll_forward(),
				KeyCode::Down => self.controls_list.scroll_backward(),
				KeyCode::Enter => self.handle_enter_shortcut(state),
				_ => {},
			}
		}
		Ok(())
	}

	fn render_ui(&self, frame: &mut Frame<'_>, _state: &ScreenState) {
		let size = frame.size();
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
}

impl WelcomeScreen {
	/// Handles the ENTER shortcut, which executes the function that the UI
	/// selector is pointing at.
	fn handle_enter_shortcut(&mut self, state: &mut ScreenState) {
		if let Some((_, item)) = self.controls_list.get_selected() {
			match item.data {
				ControlOptions::SearchGames => {
					state.set_screen_created(GameSearchScreen::default().into());
				},
				ControlOptions::ViewConfigs => state.set_screen_created(ConfigScreen.into()),
				ControlOptions::QuitApplication => state.open_status = OpenStatus::Closed,
			}
		}
	}
}
