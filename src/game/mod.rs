//! The game system in Terminal Arcade.
//! Games need two core components: a [Screen] for it, which acts as both the
//! game's UI and event handler, as well as its implementation. This module is
//! specifically reserved for the game system and the implementation. All
//! [Screen] implementations for a game goes to the [crate::ui::screens::games]
//! module.

use std::time::{
	SystemTime,
	UNIX_EPOCH,
};

use crossterm::event::Event;
use derive_builder::Builder;
use serde_derive::{
	Deserialize,
	Serialize,
};

use crate::{
	core::{
		Outcome,
		SAVE_DIR,
	},
	ui::Screen,
};

/// Gets the current UNIX time as seconds.
pub fn get_unix_time_as_secs() -> u64 {
	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// A [Game]'s metadata. Note that this does not include the game's settings.
/// This struct reads and writes from [`SAVE_DIR`]. Check out [`Self::load`]
/// and [`Self::save`] for more information.
#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize)]
pub struct GameMetadata {
	/// The game's name.
	pub name: String,

	/// The game's description.
	pub description: String,

	/// Authors of the game.
	pub authors: Vec<String>,

	/// How many times the game has been played.
	pub times_played: u64,

	/// The last time the game has been played, stored as a UNIX timestamp.
	pub last_played: Option<u64>,
}

impl GameMetadata {
	/// Gets the metadata file path, according to the game's name.
	pub fn meta_file_path(name: String) -> String {
		format!("{SAVE_DIR}/{name}.meta.toml")
	}

	/// Loads the game metadata from [`SAVE_DIR`].
	pub fn load(name: String) -> Outcome<Self> {
		let metadata_file = std::fs::read_to_string(Self::meta_file_path(name.clone()))?;
		Ok(toml::from_str::<Self>(&metadata_file)?)
	}

	/// Saves the current configuration to [`SAVE_DIR`], in TOML format.
	pub fn save(&self) -> Outcome<()> {
		let toml_string = toml::to_string_pretty(self)?;
		Ok(std::fs::write(
			Self::meta_file_path(self.name.clone()),
			toml_string,
		)?)
	}

	/// Adds 1 play count and updates the last playtime.
	pub fn play(&mut self) {
		self.times_played += 1;
		self.last_played = Some(get_unix_time_as_secs());
	}
}

/// A trait for a game in Terminal Arcade.
/// This trait is not only for the game's logic implementation, it also dictates
/// how it interacts with the rest of the Terminal Arcade UI, as well as how it
/// handles events passed to it.
/// When making a game with this trait, please also add the game to [`all_games`].
pub trait Game {
	/// Gets the metadata of the game.
	fn metadata(&self) -> GameMetadata;

	/// Indicates whether the game has finished or not.
	fn is_finished(&self) -> bool;

	/// Called when an event is passed to the game.
	fn event(&mut self, event: &Event) -> Outcome<()>;
}

/// Returns a list of all games.
pub fn all_games() -> Vec<Box<dyn Game>> {
	vec![]
}
