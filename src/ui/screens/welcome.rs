//! A module for containing the welcome screen in Terminal Arcade.

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
	core::terminal::BackendType,
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

/// The struct that welcomes the user to Terminal Arcade. To be presented every
/// time Terminal Arcade is started.
pub struct WelcomeScreen;

/// TODO: Implement the event handler for the welcome screen
impl Screen for WelcomeScreen {
	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		Self::welcome_ui(frame);
	}

	fn title(&self) -> &str {
		"Welcome to Terminal Arcade!"
	}
}

impl WelcomeScreen {
	/// Renders the welcome UI to the screen.
	fn welcome_ui(frame: &mut Frame<'_, BackendType>) {
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
		render_wcl_block(chunks[1], frame);
	}
}
