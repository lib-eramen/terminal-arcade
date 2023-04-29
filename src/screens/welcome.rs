//! A module for containing the welcome screen in Terminal Arcade.

use ansi_to_tui::IntoText;
use crossterm::{
	event::Event,
	style::Attribute,
};
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
	},
	widgets::Paragraph,
	Frame, text::Text,
};

use super::{
	util::{
		stylize,
		stylize_raw,
		ui_block,
	},
	Screen,
};
use crate::core::{
	terminal::{
		get_mut_terminal,
		BackendType,
	},
	Outcome,
};

/// Terminal Arcade's ASCII banner.
pub const BANNER: &str = r#"
/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾/‾‾/‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\            
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
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾   ‾‾  ‾‾  ‾‾  ‾‾  ‾‾‾‾‾‾  ‾‾      ‾‾  ‾‾  ‾‾‾‾‾   
"#;

/// The struct that welcomes the user to Terminal Arcade. To be presented every
/// time Terminal Arcade is started.
pub struct WelcomeScreen;

impl Screen for WelcomeScreen {
	fn on_spawn(&mut self) -> Outcome<()> {
		get_mut_terminal().draw(Self::welcome_ui)?;
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
	/// Renders the welcome UI to the screen.
	fn welcome_ui(frame: &mut Frame<'_, BackendType>) {
		let size = frame.size();
		let surrounding_block = ui_block("Terminal Arcade");
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints(
				[
					Constraint::Max(17),
					Constraint::Max(3),
					Constraint::Percentage(30),
				]
				.as_ref(),
			)
			.split(size);
		frame.render_widget(surrounding_block, size);

		let banner = Paragraph::new(stylize(BANNER)).block(ui_block("Banner")).alignment(Alignment::Center);
		frame.render_widget(banner, chunks[0]);

		let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "ersion... not found :(".to_string());
		let version_info = Paragraph::new(stylize(format!("Terminal Arcade, v{version}")))
			.block(ui_block("Version Info"))
			.alignment(Alignment::Right);
		frame.render_widget(version_info, chunks[1]);

		let controls_list =
			Paragraph::new(Self::controls_list()).block(ui_block("Controls")).alignment(Alignment::Center);
		frame.render_widget(controls_list, chunks[2]);
	}

	/// Returns a stylized controls list string.
	#[must_use]
	fn controls_list() -> Text<'static> {
		let reset = Attribute::Reset;
		format!(
			r#"
{}: Choose a game to {}! ({}){reset}
{}: View your {} ({})!{reset}
{}: {}uit...{reset} ({} and {} also work!)
"#,
			stylize_raw("[1]"),
			stylize_raw("play"),
			stylize_raw("[Ctrl-P]"),
			stylize_raw("[2]"),
			stylize_raw("settings"),
			stylize_raw("[Ctrl-Alt-S]"),
			stylize_raw("[0]"),
			stylize_raw("[Ctrl-Q]"),
			stylize_raw("[Ctrl-C]"),
			stylize_raw("[Esc]")
		).into_text().unwrap()
	}
}
