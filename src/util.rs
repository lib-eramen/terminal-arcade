//! General code utilties.

use std::ops::{
	Deref,
	DerefMut,
};

use serde::{
	Deserialize,
	Serialize,
};
use tokio::sync::mpsc::{
	unbounded_channel,
	UnboundedReceiver,
	UnboundedSender,
};

/// A wrapper struct for an unbounded [sender](UnboundedSender) and
/// [receiver](UnboundedReceiver) that can also ([mutably](DerefMut))
/// [deref](Deref)erence into a tuple of both channel sides if you need it. Used
/// as a utility for working with structs that need [`Serialize`] and
/// [`Deserialize`].
///
/// In addition, this acts as a solution to include channels in
/// a struct that needs to derive [`Serialize`] and [`Deserialize`].
/// This struct has a default implementation (which only [creates a
/// channel](unbounded_channel)) and is used when deserializing,
/// and the [inner property](Self::channel) is marked to be skipped
/// when (de)serializing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct UnboundedChannel<T> {
	#[serde(skip)]
	channel: (UnboundedSender<T>, UnboundedReceiver<T>),
}

impl<T> UnboundedChannel<T> {
	/// Constructs a new unbounded channel pair.
	pub fn new() -> Self {
		Self {
			channel: unbounded_channel(),
		}
	}

	/// Gets a reference to the sender channel.
	pub fn get_sender(&self) -> &UnboundedSender<T> {
		&self.channel.0
	}

	/// Gets a mutable reference to the sender channel.
	pub fn get_mut_sender(&mut self) -> &mut UnboundedSender<T> {
		&mut self.channel.0
	}

	/// Gets a reference to the receiver channel.
	pub fn get_receiver(&self) -> &UnboundedReceiver<T> {
		&self.channel.1
	}

	/// Gets a mutable reference to the receiver channel.
	pub fn get_mut_receiver(&mut self) -> &mut UnboundedReceiver<T> {
		&mut self.channel.1
	}
}

impl<T> Deref for UnboundedChannel<T> {
	type Target = (UnboundedSender<T>, UnboundedReceiver<T>);

	fn deref(&self) -> &Self::Target {
		&self.channel
	}
}

impl<T> DerefMut for UnboundedChannel<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.channel
	}
}

impl<T> Default for UnboundedChannel<T> {
	fn default() -> Self {
		Self::new()
	}
}
