//! The game system in Terminal Arcade.
//! Games need two core components: a [Screen] for it, which acts as both the
//! game's UI and event handler, as well as its implementation. This module is
//! specifically reserved for the game system and the implementation. All
//! [Screen] implementations for a game goes to the
//! [`crate::ui::screens::games`] module.

use std::{
	path::PathBuf,
	time::{
		Duration,
		SystemTime,
		UNIX_EPOCH,
	},
};

use chrono::{
	DateTime,
	Local,
	Utc,
};
use crossterm::event::Event;
use derive_builder::Builder;
use pluralizer::pluralize;
use serde_derive::{
	Deserialize,
	Serialize,
};

use crate::{
	core::get_save_dir,
	games::minesweeper::Minesweeper,
	ui::{
		components::scrollable_list::ItemData,
		Screen,
	},
};

pub mod minesweeper;

/// A [Vec] of games.
pub type Games = Vec<Box<dyn Game>>;

/// Gets the current UNIX time as seconds.
#[must_use]
pub fn get_unix_time_as_secs() -> u64 {
	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// Gets the metadata file path, according to the game's name.
#[must_use]
pub fn meta_file_path(name: &str) -> PathBuf {
	get_save_dir().join(format!("{}.meta.toml", name.to_lowercase()))
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

impl<'a> GameMetadata {
	/// Creates a new metadata object from a closure that builds the
	/// [`GameStaticInfo`] object, and a [`GameDynamicInfo`] object fetched by
	/// the function itself.
	pub fn new<F>(static_info: F) -> anyhow::Result<Self>
	where
		F: FnOnce(&mut GameStaticInfoBuilder) -> &mut GameStaticInfoBuilder, {
		let static_info = static_info(&mut GameStaticInfoBuilder::create_empty()).build().unwrap();
		Ok(Self {
			static_info: static_info.clone(),
			dynamic_info: GameDynamicInfo::load_or_default(&static_info.name)?,
		})
	}

	/// Returns an entry string that contains all of the metadata properties.
	#[must_use]
	pub fn get_entry_text(&self) -> String {
		format!(
			"{}: {}\n{}: {}\n{}: v{}\n{}",
			"Name",
			self.static_info.name,
			"Description",
			self.static_info.description,
			"Created at",
			self.static_info.version_created,
			self.dynamic_info.get_status_text(),
		)
	}

	/// Returns a list item usable with the [`ui::components::ScrollableList`]
	/// widget.
	#[must_use]
	pub fn get_list_entry(&self) -> ItemData {
		(
			Some(self.static_info.name.to_string()),
			self.get_entry_text(),
		)
	}

	/// Adds 1 play count and updates the last playtime, while also saving the
	/// metadata.
	pub fn play(&mut self) -> anyhow::Result<()> {
		self.dynamic_info.play();
		self.dynamic_info.save(&self.static_info.name)
	}

	/// Returns whether this game has been played.
	#[must_use]
	pub fn played(&'a self) -> bool {
		self.dynamic_info.played()
	}
}

/// A [Game]'s static, unchanging info/data. This includes things like the
/// game's name and description.
#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize)]
#[must_use]
pub struct GameStaticInfo {
	/// The game's name.
	pub name: String,

	/// The game's description.
	pub description: String,

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
	/// Formats dynamic game metadata into a human-readable string,
	#[must_use]
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_possible_wrap)]
	pub fn get_status_text(&self) -> String {
		let play_count = self.play_count;
		let last_played = self.last_played;
		if self.played() {
			let system_time = UNIX_EPOCH + Duration::from_secs(last_played.unwrap());
			let datetime = DateTime::<Local>::from(system_time);
			let date_str = datetime.format("%d/%m/%Y");

			format!(
				"Played {} {}, last played at {}",
				play_count,
				pluralize("time", play_count as isize, false),
				date_str,
			)
		} else {
			"Never played before!".to_string()
		}
	}

	/// Loads the game metadata.
	pub fn load(name: &str) -> anyhow::Result<Self> {
		let metadata_file = std::fs::read_to_string(meta_file_path(name))?;
		Ok(toml::from_str::<Self>(&metadata_file)?)
	}

	/// Saves the current configuration, in TOML format.
	pub fn save(&self, name: &str) -> anyhow::Result<()> {
		let toml_string = toml::to_string_pretty(self)?;
		Ok(std::fs::write(meta_file_path(name), toml_string)?)
	}

	/// Loads this struct from the specified location, or creates a default.
	pub fn load_or_default(name: &str) -> anyhow::Result<Self> {
		let load_results = Self::load(name);
		Ok(if let Ok(info) = load_results {
			info
		} else {
			let new = Self::default();
			std::fs::create_dir_all(get_save_dir())?;
			new.save(name)?; // So that this else branch wouldn't happen again
			new
		})
	}

	/// Adds 1 play count and updates the last playtime.
	pub fn play(&mut self) {
		self.play_count += 1;
		self.last_played = Some(get_unix_time_as_secs());
	}

	/// Checks if the game has ever been played.
	#[must_use]
	pub fn played(&self) -> bool {
		self.play_count > 0
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
	fn metadata(&self) -> GameMetadata;

	/// Indicates whether the game has finished or not.
	fn is_finished(&self) -> bool;

	/// Called when an event is passed to the game.
	fn event(&mut self, event: &Event) -> anyhow::Result<()>;

	/// The screen for the game to create.
	/// This can be either a setup screen (that can create the game screen on
	/// its own) or the game screen itself.
	fn screen_created(&self) -> Box<dyn Screen>;
}

/// Returns a list of all games.
#[must_use]
pub fn all_games() -> Games {
	vec![Box::new(Minesweeper)]
}

/// Returns a list of games that match the keyword in their name.
#[must_use]
pub fn get_games_by_keyword(keyword: &str) -> Games {
	all_games()
		.into_iter()
		.filter(|game| game.metadata().static_info.name.to_lowercase().contains(keyword))
		.collect()
}

/// Returns a list of games that match the search term. Identical to
/// [`games_by_keyword`], but if the search term is [`None`], the list of all
/// games are returned.
#[must_use]
pub fn get_games_by_search_term(term: Option<String>) -> Games {
	if let Some(ref term) = term {
		get_games_by_keyword(term)
	} else {
		all_games()
	}
}

/// Gets a game by its name.
#[must_use]
pub fn get_game_by_name(name: &str) -> Option<Box<dyn Game>> {
	all_games().into_iter().find(|game| game.metadata().static_info.name == name)
}
