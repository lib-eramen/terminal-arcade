//! Handler to manage screens and rendering them.

use std::ops::{
	Deref,
	DerefMut,
};

use color_eyre::eyre::OptionExt;
use serde::{
	Deserialize,
	Serialize,
};

use crate::{
	event::Event,
	ui::screens::{
		Screen,
		ScreenHandle,
	},
	utils::UnboundedChannel,
};

/// Handler for screens. This struct dereferenes to the inner
/// [`Vec`] of [`ScreenHandle`]s, where the top screen is named
/// "active" and should be the one rendered and receiving events.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScreenHandler {
	/// A stack of screens.
	///
	/// The top most screen (last element) renders and receives events.
	stack: Vec<ScreenHandle>,

	/// Event channel.
	event_channel: UnboundedChannel<Event>,
}

impl ScreenHandler {
	/// Creates a new screen as active.
	pub fn create_active(&mut self, screen: Screen) {
		self.push(ScreenHandle::from_screen(screen));
	}

	/// Closes the active screen.
	pub fn close_active(&mut self) -> crate::Result<ScreenHandle> {
		self.pop()
			.ok_or_eyre("no screens left in stack to close")
			.and_then(|mut handle| {
				handle.close()?;
				Ok(handle)
			})
	}

	/// Handles an incoming [`Event`].
	pub fn handle_event(&mut self, event: Event) -> crate::Result<()> {
		match event {
			Event::App(_) => todo!(),
		}
	}
}

impl Deref for ScreenHandler {
	type Target = Vec<ScreenHandle>;

	fn deref(&self) -> &Self::Target {
		&self.stack
	}
}

impl DerefMut for ScreenHandler {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.stack
	}
}
