//! A paragraph with a banner and a short message indicating that a
//! screen/component is not ready yet. Useful as a `todo!()` replacer.

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

use crate::{
	core::terminal::BackendType,
	ui::components::presets::{
		titled_ui_block,
		untitled_ui_block,
	},
};

/// A banner for an under construction message.
pub const UNDER_CONSTRUCTION_BANNER: &str = r"
        /‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾/                                   /‾‾/
       /  /‾‾‾‾‾‾‾‾‾‾/  /                                   /  / 
      /  /  ##  ##  /  /                                   /  /  
     /  /  ##  ##  /  /                                   /  /   
    /  /          /  /  /‾‾‾‾‾‾‾‾/ /‾‾‾‾‾‾‾\  /‾‾‾‾‾‾‾/  /  /    
   /  /  ######  /  /  /  /‾‾/  / /  /‾‾/  / / /‾‾‾‾‾   /  /     
  /  /  ##  ##  /  /  /  /  /  / /  /  /  / /  ‾‾‾‾‾\  /  /      
 /  /          /  /  /  /  /  / /   ‾‾‾  /  ‾‾‾‾‾/  /  ‾‾‾       
/   ‾‾‾‾‾‾‾‾‾‾‾  /  /   ‾‾‾  / /  /‾‾‾‾‾  /‾‾‾‾‾‾  / /‾‾/        
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾   ‾‾‾‾‾‾‾‾‾  ‾‾‾        ‾‾‾‾‾‾‾‾‾  ‾‾‾         
⚠ Sorry, this page is under construction!";

/// Renders the [under construction](UNDER_CONSTRUCTION_BANNER) in a block.
pub fn render_under_construction_block(frame: &mut Frame<'_>) {
	let size = frame.size();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(1)
		.constraints([Constraint::Max(13), Constraint::Max(0)])
		.split(size);
	frame.render_widget(
		titled_ui_block("Configuration (Under construction!) (Probably for a very long time!)"),
		size,
	);

	let message = Paragraph::new(UNDER_CONSTRUCTION_BANNER)
		.alignment(Alignment::Center)
		.block(untitled_ui_block().borders(Borders::NONE));
	frame.render_widget(message, chunks[0]);
}
