//! Configuration in Terminal Arcade.
//!
//! Under the [config folder](crate::util::dirs::get_config_dir) will be a list
//! of configuration files, defaulting to `config.toml`.

use std::path::PathBuf;

use color_eyre::{
	eyre::Context,
	Section,
};
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
use tracing::info;

use crate::{
	service::dirs::{
		get_config_dir,
		get_data_dir,
		AppDirs,
	},
	tui::GameSpecs,
};

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
	/// If none is found, a default one will be created at the config folder and
	/// returned. If one is found, the function tries to deserialize it and
	/// returns the resulting config.
	pub fn fetch() -> crate::Result<Self> {
		let config_dir = get_config_dir();
		let data_dir = get_data_dir();
		let mut config_builder = ConfigBuilder::<DefaultState>::default()
			.set_default("config_dir", config_dir.to_str())?
			.set_default("data_dir", data_dir.to_str())?;

		let config_path = config_dir.join("config.toml");
		if !config_path.exists() {
			info!(
				expected_path = config_path.clone().display().to_string(),
				"no config exists; using default config"
			);
			return Self::create_default(config_path);
		}

		config_builder = config_builder
			.add_source(
				config::File::from(config_path.clone())
					.format(FileFormat::Toml)
					.required(true),
			)
			.add_source(config::Environment::with_prefix("TA"));

		config_builder
			.build()?
			.try_deserialize()
			.wrap_err("unable to parse & deserialize config")
			.warning(
				"your config might have been modified - it is missing fields, \
				 misformatted, etc.",
			)
			.with_suggestion(|| {
				format!("check your config at {}!", config_path.display())
			})
	}

	/// Creates a new default config with the provided path,
	/// returning said default config in the process.
	pub fn create_default(path: PathBuf) -> crate::Result<Config> {
		let default_config = Self::default();
		default_config.save(path)?;
		Ok(default_config)
	}

	/// Saves the current config to the provided path.
	pub fn save(&self, path: PathBuf) -> crate::Result<()> {
		info!(
			config = ?self,
			path = path.clone().display().to_string(),
			"writing config to file"
		);
		std::fs::write(path, toml::to_string(self)?)?;
		Ok(())
	}
}
