//! Utilities for working with [`Event`]s.

use tokio::sync::mpsc::{
	error::SendError,
	UnboundedSender,
};
use tracing::info;

use crate::events::{
	AppEvent,
	Event,
	InputEvent,
	TuiEvent,
};

/// A middleman that receives events from the [`Tui`], and buffers the
/// [`InputEvent`]s to be sent every [`AppEvent::Tick`] and sends back
/// [`AppEvent`]s through a cloned [`UnboundedSender`].
#[derive(Debug)]
pub struct TuiAppMiddleman {
	/// Buffer for [`InputEvent`]s.
	input_buffer: Vec<InputEvent>,

	/// Event channel.
	event_sender: UnboundedSender<Event>,
}

impl TuiAppMiddleman {
	/// Constructs a new [`Tui`]-[`App`] middleman.
	pub fn new(event_sender: UnboundedSender<Event>) -> Self {
		Self {
			input_buffer: Vec::new(),
			event_sender,
		}
	}

	/// Takes a [`Tui`] event and either buffers it or passes it on to the
	/// [`Self::event_channel`].
	pub fn handle_tui_event(
		&mut self,
		event: TuiEvent,
	) -> Result<(), SendError<Event>> {
		match event {
			TuiEvent::Hello => {
				info!(
					"the middleman does not get paid enough to translate. yes \
					 i can hear you, tui."
				);
			},
			TuiEvent::Tick => {
				self.event_sender.send(
					AppEvent::Tick(self.input_buffer.drain(..).collect())
						.into(),
				)?;
			},
			TuiEvent::Render => {
				self.event_sender.send(AppEvent::Render.into())?;
			},
			TuiEvent::Input(input_event) => {
				self.input_buffer.push(input_event);
			},
		}
		Ok(())
	}
}
