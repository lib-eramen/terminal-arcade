//! The game system in Terminal Arcade.
//! Games need two core components: a [Screen] for it, which acts as both the
//! game's UI and event handler, as well as its implementation. This module is
//! specifically reserved for the game system and the implementation. All
//! [Screen] implementations for a game goes to the
//! [`crate::ui::screens::games`] module.

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
#[must_use]
pub fn get_unix_time_as_secs() -> u64 {
	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// Gets the metadata file path, according to the game's name.
#[must_use]
pub fn meta_file_path(name: &str) -> String {
	format!("{SAVE_DIR}/{name}.meta.toml")
}

/// A [Game]'s metadata. Note that this does not include the game's settings.
/// This struct reads and writes from [`SAVE_DIR`]. Check out [`Self::load`]
/// and [`Self::save`] for more information.
#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize)]
#[must_use]
pub struct GameMetadata {
	/// The game's static information.
	pub static_info: GameStaticInfo,

	/// The game's dynamic information.
	pub dynamic_info: GameDynamicInfo,
}

impl GameMetadata {
	/// Creates a new metadata object from a closure that builds the
	/// [`GameStaticInfo`] object, and a [`GameDynamicInfo`] object fetched by
	/// the function itself.
	pub fn new<F>(static_info: F, name: &str) -> Self
	where
		F: FnOnce(&mut GameStaticInfoBuilder) -> &mut GameStaticInfoBuilder, {
		let static_info = static_info(&mut GameStaticInfoBuilder::create_empty()).build().unwrap();
		Self {
			static_info,
			dynamic_info: GameDynamicInfo::load_or_default(name),
		}
	}
}

/// A [Game]'s static, unchanging info/data. This includes things like the
/// game's name, description, and authors.
#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize)]
#[must_use]
pub struct GameStaticInfo {
	/// The game's name.
	pub name: String,

	/// The game's description.
	pub description: String,

	/// Authors of the game.
	pub authors: Vec<String>,

	/// The version that the game was created on.
	pub version_created: String,
}

/// A [Game]'s dynamic info, such as the game's play count, or the last played
/// date of the game.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[must_use]
pub struct GameDynamicInfo {
	/// The game's play count.
	pub play_count: u64,

	/// The game's [Option]al last-played UNIX timestamp.
	pub last_played: Option<u64>,
}

impl GameDynamicInfo {
	/// Loads the game metadata from [`SAVE_DIR`].
	pub fn load(name: &str) -> Outcome<Self> {
		let metadata_file = std::fs::read_to_string(meta_file_path(name))?;
		Ok(toml::from_str::<Self>(&metadata_file)?)
	}

	/// Saves the current configuration to [`SAVE_DIR`], in TOML format.
	pub fn save(&self, name: &str) -> Outcome<()> {
		let toml_string = toml::to_string_pretty(self)?;
		Ok(std::fs::write(meta_file_path(name), toml_string)?)
	}

	/// Loads this struct from the specified location, or creates a default.
	pub fn load_or_default(name: &str) -> Self {
		let load_results = Self::load(name);
		if let Ok(info) = load_results {
			info
		} else {
			let new = Self::default();
			new.save(name).unwrap(); // So that this else branch wouldn't happen again
			new
		}
	}

	/// Adds 1 play count and updates the last playtime.
	pub fn play(&mut self) {
		self.play_count += 1;
		self.last_played = Some(get_unix_time_as_secs());
	}
}

/// A trait for a game in Terminal Arcade.
/// This trait is not only for the game's logic implementation, it also dictates
/// how it interacts with the rest of the Terminal Arcade UI, as well as how it
/// handles events passed to it.
/// When making a game with this trait, please also add the game to
/// [`all_games`].
pub trait Game {
	/// Gets the metadata of the game.
	fn metadata(&self) -> &GameMetadata;

	/// Indicates whether the game has finished or not.
	fn is_finished(&self) -> bool;

	/// Called when an event is passed to the game.
	fn event(&mut self, event: &Event) -> Outcome<()>;
}

/// Returns a list of all games.
#[must_use]
pub fn all_games() -> Vec<Box<dyn Game>> {
	vec![]
}
