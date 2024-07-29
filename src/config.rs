//! Configuration in Terminal Arcade.
//!
//! Under the [config folder](crate::util::dirs::get_config_dir) will be a list
//! of configuration files, defaulting to `config.toml`.

use std::path::PathBuf;

use config::{
	builder::DefaultState,
	ConfigBuilder,
	FileFormat,
};
use derive_new::new;
use serde::{
	Deserialize,
	Serialize,
};

use crate::service::dirs::{
	get_config_dir,
	get_data_dir,
	AppDirs,
};

/// Wrapper struct around two [`f64`]s for the ticks per second and the frames
/// per second numbers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, new)]
pub struct GameSpecs {
	/// Ticks per second.
	pub tps: f64,

	/// Frames per second.
	pub fps: f64,
}

impl Default for GameSpecs {
	fn default() -> Self {
		Self::new(64.0, 60.0)
	}
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, new)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
	/// Directories that Terminal Arcade depends on.
	pub app_dirs: AppDirs,

	/// Terminal Arcade's game specifications.
	pub game_specs: GameSpecs,
}

impl Config {
	/// Fetches a new configuration object for Terminal Arcade.
	///
	/// If none is found, a default one will be created at the config folder and
	/// returned.
	pub fn fetch() -> crate::Result<Self> {
		let config_dir = get_config_dir();
		let data_dir = get_data_dir();
		let mut config_builder = ConfigBuilder::<DefaultState>::default()
			.set_default("config_dir", config_dir.to_str())?
			.set_default("data_dir", data_dir.to_str())?;

		let config_path = config_dir.join("config.toml");
		if !config_path.exists() {
			let config = Self::default();
			config.save(config_path)?;
			return Ok(config);
		}

		config_builder = config_builder
			.add_source(
				config::File::from(config_path)
					.format(FileFormat::Toml)
					.required(true),
			)
			.add_source(config::Environment::with_prefix("TA"));
		let result = config_builder.build()?.try_deserialize()?;
		Ok(result)
	}

	/// Saves the current config to the provided path.
	pub fn save(&self, path: PathBuf) -> crate::Result<()> {
		std::fs::write(path, toml::to_string(self)?)?;
		Ok(())
	}
}

// TODO: Add config validation
