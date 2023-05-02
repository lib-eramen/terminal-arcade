//! A paragraph with a banner and a short message indicating that a
//! screen/component is not ready yet. Useful as a `todo!()` replacer.
//! Note that in here, UC stands for "under construction".

use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
	},
	widgets::{
		Borders,
		Paragraph,
	},
	Frame,
};

use super::presets::{
	titled_ui_block,
	untitled_ui_block,
};
use crate::{
	core::terminal::BackendType,
	ui::util::stylize,
};

/// A banner for an under construction message.
pub const UC_BANNER: &str = r#"
        /‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾/  "When they tell you to build stuff, build bomb-ass
       /  /‾‾‾‾‾‾‾‾‾‾/  /  banners in place of it. At least you'll have some- 
      /  /  ##  ##  /  /  thing to show when HR inevitably writes you up for  
     /  /  ##  ##  /  /  not being productive."                 Ramen, 2023   
    /  /          /  /  /‾‾‾‾‾‾‾‾/ /‾‾‾‾‾‾‾\  /‾‾‾‾‾‾‾/  /‾‾/ /‾‾/ /‾‾/ /‾‾/  
   /  /  ######  /  /  /  /‾‾/  / /  /‾‾/  / / /‾‾‾‾‾   /  / /  / /  / /  /   
  /  /  ##  ##  /  /  /  /  /  / /  /  /  / /  ‾‾‾‾‾\  /  / /  / /  / /  /    
 /  /          /  /  /  /  /  / /   ‾‾‾  /  ‾‾‾‾‾/  /  ‾‾‾  ‾‾‾  ‾‾‾  ‾‾‾     
/   ‾‾‾‾‾‾‾‾‾‾‾  /  /   ‾‾‾  / /  /‾‾‾‾‾  /‾‾‾‾‾‾  / /‾‾/ /‾‾/ /‾‾/ /‾‾/      
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾   ‾‾‾‾‾‾‾‾‾  ‾‾‾        ‾‾‾‾‾‾‾‾‾  ‾‾‾  ‾‾‾  ‾‾‾  ‾‾‾       
⚠ Sorry! This page is under construction! I promise it's coming soon (read: probably not for a while)! Press [ESC] to leave for now..."#;

/// Draws the [under construction](UC_BANNER) in a block.
pub fn draw_uc_block(frame: &mut Frame<'_, BackendType>) {
	let size = frame.size();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(1)
		.constraints([
			Constraint::Max(13), // Banner height
			Constraint::Max(0) // Prevents blocks from taking up all remaining space
		].as_ref())
		.split(size);
	frame.render_widget(
		titled_ui_block("Configuration (Under construction!) (Probably for a very long time!)"),
		size,
	);

	let message = Paragraph::new(stylize(UC_BANNER))
		.alignment(Alignment::Center)
		.block(untitled_ui_block().borders(Borders::NONE));
	frame.render_widget(message, chunks[0]);
}
