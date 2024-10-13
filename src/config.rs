//! Configuration for the app.

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

use crate::{
	services::{
		files::AppFiles,
		CARGO_PKG_NAME,
	},
	tui::GameSpecs,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, new)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
	/// App files.
	#[serde(skip)]
	pub app_files: AppFiles,

	/// Game specifications.
	pub game_specs: GameSpecs,
}

impl Config {
	/// Fetches a new configuration object for the app.
	/// If none is found, a default one will be created at the config folder and
	/// returned. If one is found, the function tries to deserialize it and
	/// returns the resulting config.
	pub fn fetch(app_files: AppFiles) -> crate::Result<Self> {
		let config_dir = app_files.get_config_path(None)?;
		let mut config_builder = ConfigBuilder::<DefaultState>::default();

		let config_path = config_dir.join("config.toml");
		if !config_path.exists() {
			tracing::info!(
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
			.add_source(config::Environment::with_prefix(&CARGO_PKG_NAME));

		let mut config = config_builder
			.build()?
			.try_deserialize::<Self>()
			.wrap_err("unable to parse & deserialize config")
			.warning(
				"your config might have been modified - it is missing fields, \
				 malformatted, etc.",
			)
			.with_suggestion(|| {
				format!("check your config at {}!", config_path.display())
			})?;
		config.app_files = app_files;
		Ok(config)
	}

	/// Constructs a new default config with the provided path,
	/// returning said default config in the process.
	pub fn create_default(path: PathBuf) -> crate::Result<Config> {
		let default_config = Self::default();
		default_config.save(path)?;
		Ok(default_config)
	}

	/// Saves the current config to the provided path.
	pub fn save(&self, path: PathBuf) -> crate::Result<()> {
		tracing::info!(
			config = ?self,
			path = path.clone().display().to_string(),
			"writing config to file"
		);
		std::fs::write(path, toml::to_string(self)?)?;
		Ok(())
	}
}
