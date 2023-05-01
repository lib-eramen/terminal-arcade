//! A module for containing the welcome screen in Terminal Arcade.

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
	widgets::Paragraph,
	Frame,
};

use crate::{
	core::{
		terminal::BackendType,
		Outcome,
	},
	ui::{
		components::{
			presets::{
				titled_ui_block,
				untitled_ui_block,
			},
			wcl::render_wcl_block,
		},
		util::{
			get_crate_version,
			stylize,
		},
		Screen,
	},
};

/// Terminal Arcade's ASCII banner.
pub const BANNER: &str = r#"/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾/‾‾/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\            
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
/                             /  / / / / / / / / /  ‾‾‾/ / /‾‾‾‾\ \ \  / /  ‾‾/  
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾   ‾‾  ‾‾  ‾‾  ‾‾  ‾‾‾‾‾‾  ‾‾      ‾‾  ‾‾  ‾‾‾‾‾   "#;

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
#[derive(Default)]
pub struct WelcomeScreen {
	/// Whether this screen is closing or not.
	closing: bool,

	/// The screen that this screen is spawning.
	screen_created: Option<ScreenCreated>,

	/// The control index that is being selected and highlighted on the UI.
	/// 0 indicates the game selection control, 1 indicates the settings, and 2
	/// indicates quit. Note that this index is in no way related to the actual
	/// shortcut listed on the controls list, it is related to the order in
	/// which the controls are presented on the UI.
	selected_control: Option<u8>,
}

impl Screen for WelcomeScreen {
	fn is_closing(&self) -> bool {
		self.closing
	}

	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		self.welcome_ui(frame);
	}

	fn title(&self) -> &str {
		"Welcome to Terminal Arcade!"
	}

	fn event(&mut self, event: &Event) -> Outcome<()> {
		if let Event::Key(key) = event {
			match key.code {
				KeyCode::Char(character) => {
					if key.modifiers != KeyModifiers::NONE {
						return Ok(());
					}
					self.handle_char_shortcut(character);
				},
				KeyCode::Up => self.handle_up_shortcut(),
				KeyCode::Down => self.handle_down_shortcut(),
				KeyCode::Enter => self.handle_enter_shortcut(),
				_ => {},
			}
		}
		Ok(())
	}

	fn screen_created(&self) -> Option<Box<dyn Screen>> {
		self.screen_created.map(|screen| match screen {
			ScreenCreated::GameSelection => Self::create_game_selection_screen(),
			ScreenCreated::Settings => Self::create_settings_screen(),
		})
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
		todo!()
	}

	// TODO: Game selection screen.
	/// Creates the settings screen to switch to from this screen.
	fn create_settings_screen() -> Box<dyn Screen> {
		todo!()
	}

	/// Handles the shortcut associated with the character inputted.
	fn handle_char_shortcut(&mut self, character: char) {
		match character {
			'1' | 'p' => self.set_screen_created(ScreenCreated::GameSelection),
			'2' | 's' => self.set_screen_created(ScreenCreated::Settings),
			'0' | 'q' => self.mark_closed(),
			_ => {},
		}
	}

	/// Handles the UP-arrow shortcut, which moves the UI selector up.
	fn handle_up_shortcut(&mut self) {
		self.selected_control = Some(
			if let Some(index) = self.selected_control {
				if index == 0 {
					2
				} else {
					index - 1
				}
			} else {
				0
			},
		);
	}

	/// Handles the DOWN-arrow shortcut, which moves the UI selector down.
	fn handle_down_shortcut(&mut self) {
		self.selected_control = Some(
			if let Some(index) = self.selected_control {
				if index == 2 {
					0
				} else {
					index + 1
				}
			} else {
				0
			},
		);
	}

	/// Handles the ENTER shortcut, which executes the function that the UI
	/// selector is pointing at.
	fn handle_enter_shortcut(&mut self) {
		if let Some(index) = self.selected_control {
			match index {
				0 => self.set_screen_created(ScreenCreated::GameSelection),
				1 => self.set_screen_created(ScreenCreated::Settings),
				2 => self.mark_closed(),
				_ => panic!("Index not in predefined range (0..2) of welcome controls!"),
			}
		}
	}

	/// Renders the welcome UI to the screen.
	fn welcome_ui(&self, frame: &mut Frame<'_, BackendType>) {
		let size = frame.size();
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints([Constraint::Max(17), Constraint::Max(11), Constraint::Min(0)].as_ref())
			.split(size);
		frame.render_widget(titled_ui_block("Welcome to Terminal Arcade!"), size);
		let banner_text = stylize(format!(
			"{}\nTerminal Arcade, {}",
			BANNER,
			get_crate_version()
		));
		let banner =
			Paragraph::new(banner_text).block(untitled_ui_block()).alignment(Alignment::Center);
		frame.render_widget(banner, chunks[0]);
		render_wcl_block(chunks[1], frame, self.selected_control);
	}
}
