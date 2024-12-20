//! Utilities for working with [`Event`]s.

use tokio::sync::mpsc::{
	error::SendError,
	UnboundedSender,
};

use crate::events::{
	AppEvent,
	Event,
	InputEvent,
	StationEvent,
};

/// A middleman that receives events from the [`Station`], and buffers the
/// [`InputEvent`]s to be sent every [`AppEvent::Tick`] and sends back
/// [`AppEvent`]s through a cloned [`UnboundedSender`].
#[derive(Debug)]
pub struct Turnstile {
	/// Buffer for [`InputEvent`]s.
	input_buffer: Vec<InputEvent>,

	/// Event channel.
	event_sender: UnboundedSender<Event>,
}

impl Turnstile {
	/// Constructs a new [`Station`]-[`App`] middleman.
	pub fn new(event_sender: UnboundedSender<Event>) -> Self {
		Self {
			input_buffer: Vec::new(),
			event_sender,
		}
	}

	/// Takes a [`Station`] event and either buffers it or passes it on to the
	/// [`Self::event_channel`].
	pub fn handle_station_event(
		&mut self,
		event: StationEvent,
	) -> Result<(), SendError<Event>> {
		match event {
			StationEvent::Hello => {
				tracing::info!(
					"the middleman does not get paid enough to translate. yes \
					 i can hear you, station."
				);
			},
			StationEvent::Tick => {
				self.event_sender.send(
					AppEvent::Tick(self.input_buffer.drain(..).collect())
						.into(),
				)?;
			},
			StationEvent::Render => {
				self.event_sender.send(AppEvent::Render.into())?;
			},
			StationEvent::Input(input_event) => {
				self.input_buffer.push(input_event);
			},
		}
		Ok(())
	}
}
