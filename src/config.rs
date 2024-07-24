//! Configuration in Terminal Arcade.
//!
//! Under the [config folder](crate::util::dirs::get_config_dir) will be a list
//! of configuration files, defaulting to `config.toml`.

use config::{
	builder::DefaultState,
	ConfigBuilder,
	FileFormat,
};
use serde::{
	Deserialize,
	Serialize,
};

use crate::util::dirs::{
	get_config_dir,
	get_data_dir,
	AppDirs,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
	/// Directories that Terminal Arcade depends on.
	#[serde(flatten)]
	app_dirs: AppDirs,
}

impl Config {
	/// Constructs a new configuration object for Terminal Arcade.
	///
	/// If none is found, a default one will be created at the config folder and
	/// returned.
	pub fn new() -> crate::Result<Self> {
		let config_dir = get_config_dir();
		let data_dir = get_data_dir();
		let mut config_builder = ConfigBuilder::<DefaultState>::default()
			.set_default("config_dir", config_dir.to_str())?
			.set_default("data_dir", data_dir.to_str())?;

		// TODO: Remove hardcoded "config.toml"
		let config_path = config_dir.join("config.toml");
		if !config_path.exists() {
			let config = Self::default();
			std::fs::write(config_path, toml::to_string(&config)?)?;
			return Ok(config);
		}

		config_builder = config_builder
			.add_source(
				config::File::from(config_path)
					.format(FileFormat::Toml)
					.required(true),
			)
			.add_source(config::Environment::with_prefix("TA"));
		let result = dbg!(config_builder.build()?.try_deserialize()?);
		Ok(result)
	}
}
