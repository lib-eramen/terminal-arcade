//! A wrapper struct for an unbounded [sender](UnboundedSender) and
//! [receiver](UnboundedReceiver) that can also ([mutably](DerefMut))
//! [deref](Deref)erence into a tuple of both channel sides if you need it. Used
//! as a utility for working with structs that need [`Serialize`] and
//! [`Deserialize`].
//!
//! In addition, this acts as a solution to include channels in
//! a struct that needs to derive [`Serialize`] and [`Deserialize`].
//! This struct has a default implementation (which only [creates a
//! channel](unbounded_channel)) and is used when deserializing,
//! and the [inner property](Self::channel) is marked to be skipped
//! when (de)serializing.

use std::ops::{
	Deref,
	DerefMut,
};

use serde::{
	Deserialize,
	Serialize,
};
use tokio::sync::mpsc::{
	error::{
		SendError,
		TryRecvError,
	},
	unbounded_channel,
	UnboundedReceiver,
	UnboundedSender,
};

/// A wrapper struct for an [`unbounded_channel`] that serializes into nothing
/// and deserializes into a new channel. Used to appease [`serde`].
///
/// See the [module-level docs](self) for more info.
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

	/// Sends something with the sender channel.
	pub fn send(&self, thing: T) -> Result<(), SendError<T>>
	where
		T: Send + Sync + 'static,
	{
		self.get_sender().send(thing)
	}

	/// Tries to receive an event with the receiver channel.
	/// For more info, see [the delegated method's
	/// docs](UnboundedReceiver::try_recv).
	pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
		self.get_mut_receiver().try_recv()
	}

	/// Receives an event with the receiver channel.
	/// For more info, see [the delegated method's
	/// docs](UnboundedReceiver::recv).
	#[allow(unused, reason = "completeness")]
	pub async fn recv(&mut self) -> Option<T> {
		self.get_mut_receiver().recv().await
	}

	/// Gets a reference to the sender channel.
	pub fn get_sender(&self) -> &UnboundedSender<T> {
		&self.channel.0
	}

	/// Gets a mutable reference to the sender channel.
	#[allow(unused)]
	pub fn get_mut_sender(&mut self) -> &mut UnboundedSender<T> {
		&mut self.channel.0
	}

	/// Gets a reference to the receiver channel.
	#[allow(unused, reason = "completeness")]
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
