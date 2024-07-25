//! Utilities for setting up directories in Terminal Arcade, using
//! [`directories`].

use std::{
	fmt::Display,
	path::PathBuf,
};

use derive_new::new;
use directories::ProjectDirs;
use serde::{
	Deserialize,
	Serialize,
};
use tracing::info;

use crate::util::PROJECT_NAME;

lazy_static::lazy_static! {
	static ref DATA_FOLDER_ENV_VAR: String =
		format!("{}_DATA_PATH", PROJECT_NAME.to_uppercase());

	static ref CONFIG_FOLDER_ENV_VAR: String =
		format!("{}_CONFIG_PATH", PROJECT_NAME.to_uppercase());

	static ref DATA_FOLDER: Option<PathBuf> =
		std::env::var(DATA_FOLDER_ENV_VAR.clone())
			.ok()
			.map(PathBuf::from);

	static ref CONFIG_FOLDER: Option<PathBuf> =
		std::env::var(CONFIG_FOLDER_ENV_VAR.clone())
			.ok()
			.map(PathBuf::from);
}

/// Source for where a folder is found or used for Terminal Arcade.
pub enum PathSource {
	/// Via in environment variable with the name provided.
	Environment(String),

	/// Via locally sourced directories, specific to the project through what
	/// [`project_dir`] generates.
	Local,

	/// Default option - in the same folder where the program is run.
	Default,
}

/// Directories used in Terminal Arcade.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct AppDirs {
	/// Directory holding Terminal Arcade's config.
	pub config_dir: PathBuf,

	/// Directory holding Terminal Arcade's data.
	pub data_dir: PathBuf,
}

impl Default for AppDirs {
	fn default() -> Self {
		Self::new(get_config_dir(), get_data_dir())
	}
}

impl Display for PathSource {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(
			match self {
				Self::Environment(var_name) => {
					format!("env variable: {var_name}")
				},
				Self::Local => "local dirs".to_string(),
				Self::Default => "default location (cwd)".to_string(),
			}
			.as_str(),
		)
	}
}

/// Retrieves a [`directories`]-generated [set of paths](ProjectDirs) for use in
/// Terminal Arcade.
fn project_dir() -> Option<ProjectDirs> {
	ProjectDirs::from("", "", "terminal-arcade")
}

/// Gets the config directory to be used for the current Terminal Arcade
/// session.
pub fn get_config_dir() -> PathBuf {
	let (path, source) = if let Some(path) = CONFIG_FOLDER.clone() {
		(
			path.clone(),
			PathSource::Environment(CONFIG_FOLDER_ENV_VAR.clone()),
		)
	} else if let Some(project_dir) = project_dir() {
		(project_dir.config_local_dir().into(), PathSource::Local)
	} else {
		(PathBuf::from(".").join(".config"), PathSource::Default)
	};
	info!("found config dir via {source}, at {}", path.display());
	path
}

/// Gets the data directory to be used for the current Terminal Arcade session.
pub fn get_data_dir() -> PathBuf {
	let (path, source) = if let Some(path) = DATA_FOLDER.clone() {
		(
			path.clone(),
			PathSource::Environment(DATA_FOLDER_ENV_VAR.clone()),
		)
	} else if let Some(project_dir) = project_dir() {
		(project_dir.data_local_dir().into(), PathSource::Local)
	} else {
		(PathBuf::from(".").join(".data"), PathSource::Default)
	};
	info!("found data dir via {source}, at {}", path.display());
	path
}

/// Initializes directories that are used in Terminal Arcade.
pub fn init_project_dirs() -> crate::Result<()> {
	use std::fs::create_dir_all;
	info!("initializing project dirs");
	create_dir_all(get_config_dir())?;
	create_dir_all(get_data_dir())?;
	Ok(())
}
