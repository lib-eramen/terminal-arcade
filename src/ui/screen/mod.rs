//! [`Screen`]s - a core construct in Terminal Arcade for rendering and
//! receiving input. A screen (usually used through a [`ScreenHandle`] which is
//! where ) receives and produces events.

use std::fmt::Debug;

use ratatui::{
	layout::{
		Alignment,
		Rect,
	},
	prelude::Stylize,
	style::Style,
	widgets::{
		block::Title,
		Block,
	},
	Frame,
};

use crate::{
	events::{
		Event,
		ScreenEvent,
	},
	ui::{
		components::widgets::{
			blocks::titled_block,
			HIGHLIGHTED,
		},
		screen::{
			ScreenHandleData,
			ScreenMetadataBuilder,
		},
		UiElement,
	},
};

pub mod handle;
pub mod metadata;

pub use handle::*;
pub use metadata::*;

// FUTURE: When `typetag` supports associated types, switch to an `Either` API
// or the sorts with the events.

/// A screen that holds state, receives events and renders to the terminal.
/// The screen requires implementing [`UiElement`] for rendering & event
/// handling, and does a few handling on top of it.
pub trait Screen:
	UiElement<State = ScreenHandleData> + Debug + Send + Sync
{
	/// Returns the initial state that's associated with the screen.
	fn get_init_state<'a>(
		&self,
		builder: &'a mut ScreenMetadataBuilder,
	) -> &'a mut ScreenMetadataBuilder;

	/// Performs closing actions for the screen.
	/// The default behavior is just to send an event to finish the screen.
	fn close(&mut self, handle: &mut ScreenHandleData) -> crate::Result<()> {
		handle.event_sender.send(ScreenEvent::Finish.into())?;
		Ok(())
	}

	/// Runs the screen's state through one tick.
	fn tick(&mut self, handle: &mut ScreenHandleData) -> crate::Result<()> {
		UiElement::tick(self, handle)
	}

	/// Handles an incoming [`Event`].
	fn event(
		&mut self,
		handle: &mut ScreenHandleData,
		event: &Event,
	) -> crate::Result<()> {
		UiElement::event(self, handle, event)
	}

	/// Renders this screen.
	fn render(
		&self,
		handle: &mut ScreenHandleData,
		frame: &mut Frame<'_>,
		area: Rect,
	) {
		let base_screen_block =
			base_screen_block(handle.metadata.lock().unwrap().title.clone());
		frame.render_widget(base_screen_block, area);
		UiElement::render(self, handle, frame.buffer_mut(), area);
	}
}

/// A base block for a [`Screen`](crate::ui::screens::Screen), with a
/// colorred border and [`HIGHLIGHTED`] title.
fn base_screen_block<'a, T: Into<Title<'a>>>(title: T) -> Block<'a> {
	titled_block(title)
		.border_style(Style::default().blue())
		.title_style(HIGHLIGHTED)
		.title_alignment(Alignment::Center)
}
