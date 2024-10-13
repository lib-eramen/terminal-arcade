//! Utilities for setting up directories in Terminal Arcade, using
//! [`directories`].

use std::{
	fmt::Display,
	ops::{
		Deref,
		DerefMut,
	},
	path::{
		Path,
		PathBuf,
	},
};

use color_eyre::{
	eyre::{
		eyre,
		Context,
	},
	Section,
};
use directories::ProjectDirs;

use crate::services::{
	CARGO_PKG_NAME,
	PROJECT_NAME,
};

lazy_static::lazy_static! {
	static ref DATA_FOLDER_ENV_VAR: String =
		format!("{}_DATA_PATH", PROJECT_NAME.to_uppercase());

	static ref CONFIG_FOLDER_ENV_VAR: String =
		format!("{}_CONFIG_PATH", PROJECT_NAME.to_uppercase());
}

/// Source for where a folder is found or used for Terminal Arcade.
pub enum PathSource {
	/// Via in environment variable with the name provided.
	EnvVar(String),

	/// Via locally sourced directories, specific to the project through what
	/// [`project_dir`] generates.
	Local,

	/// Fallback option (the current working directory).
	Fallback,
}

impl Display for PathSource {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(
			match self {
				Self::EnvVar(var_name) => {
					format!("env variable: {var_name}")
				},
				Self::Local => "local dirs".to_string(),
				Self::Fallback => "fallback location (cwd)".to_string(),
			}
			.as_str(),
		)
	}
}
/// Project directories for Terminal Arcade.
#[derive(Debug, Clone)]
pub struct AppDirs(Option<ProjectDirs>);

impl AppDirs {
	/// Constructs a new [`ProjectDirs`] object with [`CARGO_PKG_NAME`] as the
	/// name.
	pub fn new(name: &str) -> Self {
		let project_dirs = ProjectDirs::from("", "", name);
		tracing::info!(dirs = ?project_dirs, "constructed app-project-dirs");
		Self(project_dirs)
	}

	/// Returns the path if it [exists](PathBuf::exists), and errors otherwise.
	fn get_existing_path(path: PathBuf) -> crate::Result<PathBuf> {
		if path.exists() {
			Ok(path)
		} else {
			Err(eyre!("env path {} does not exist!", path.display()))
		}
	}

	/// Gets a path from an environment variable, also checking whether it
	/// exists.
	fn get_env_var_dir(var: &str) -> crate::Result<PathBuf> {
		let path = PathBuf::from(std::env::var(var)?);
		Self::get_existing_path(path).map_err(|err| {
			err.with_note(|| format!("read from env var: {var}"))
		})
	}

	/// Gets a directory to be used for the current Terminal Arcade session,
	/// based on three criteria with descending prioirity: the environment
	/// variable, the "local" (location in a user folder) folder, and the
	/// fallback being the current working directory.
	pub fn get_dir_from_sources<F>(
		&self,
		env_folder_var: &str,
		get_project_dir_path: F,
	) -> std::io::Result<(PathBuf, PathSource)>
	where
		F: Fn(&ProjectDirs) -> &Path,
	{
		Ok(match (Self::get_env_var_dir(env_folder_var), &self.0) {
			(Ok(env_path), _) => {
				(env_path, PathSource::EnvVar(env_folder_var.to_string()))
			},
			(_, Some(project_dirs)) => (
				get_project_dir_path(project_dirs).to_path_buf(),
				PathSource::Local,
			),
			(Err(err), None) => {
				tracing::error!(
					err = err.root_cause(),
					"while trying to read directory from env var"
				);
				(std::env::current_dir()?, PathSource::Fallback)
			},
		})
	}

	/// Gets a directory to be used for the current app session.
	#[tracing::instrument(
		name = "get-app-dir",
		skip(self, get_project_dir_path)
	)]
	pub fn get_app_dir<F>(
		&self,
		env_folder_var: &str,
		get_project_dir_path: F,
		subdir: Option<PathBuf>,
	) -> std::io::Result<(PathBuf, PathSource)>
	where
		F: Fn(&ProjectDirs) -> &Path,
	{
		let (mut path, source) =
			self.get_dir_from_sources(env_folder_var, get_project_dir_path)?;
		if let Some(subdir) = subdir {
			path = path.join(subdir);
		}
		Ok((path, source))
	}

	/// [Gets an app directory](`Self::get_app_dir`) and checks if the resulting
	/// path exists.
	#[expect(unused, reason = "maybe one day this is helpful? idk")]
	pub fn get_existing_app_dir<F>(
		&self,
		env_folder_var: &str,
		get_project_dir_path: F,
		subdir: Option<PathBuf>,
	) -> crate::Result<(PathBuf, PathSource)>
	where
		F: Fn(&ProjectDirs) -> &Path,
	{
		self.get_app_dir(env_folder_var, get_project_dir_path, subdir)
			.wrap_err("io error while retrieving app dir")
			.and_then(|(path, source)| {
				Ok((Self::get_existing_path(path)?, source))
			})
	}

	/// [Gets an app directory](`Self::get_app_dir`) or creates the directory.
	/// The `purpose` parameter is interpolated in a log message as follows:
	/// `"found {purpose} dir"`.
	///
	/// Error information from [`std::fs::create_dir_all`] is discarded. Sorry.
	/// I couldn't bother.
	pub fn get_or_create_app_dir<F>(
		&self,
		purpose: &str,
		env_folder_var: &str,
		get_project_dir_path: F,
		subdir: Option<PathBuf>,
	) -> crate::Result<(PathBuf, PathSource)>
	where
		F: Fn(&ProjectDirs) -> &Path,
	{
		let (path, source) =
			self.get_app_dir(env_folder_var, get_project_dir_path, subdir)?;
		let path_display = path.display().to_string();
		tracing::info!(
			%source,
			path = path_display,
			"finding {purpose} dir"
		);
		if !path.exists() {
			tracing::info!(
				path = path_display,
				"{path_display} does not exist; creating now"
			);
			let _ = std::fs::create_dir_all(&path);
		}
		Ok((path, source))
	}

	/// Gets a directory from the app's [config
	/// directory](ProjectDirs::config_dir).
	pub fn get_config_dir(
		&self,
		purpose: &str,
		subdir: Option<PathBuf>,
	) -> crate::Result<(PathBuf, PathSource)> {
		self.get_or_create_app_dir(
			purpose,
			&CONFIG_FOLDER_ENV_VAR,
			|dirs| dirs.config_dir(),
			subdir,
		)
	}

	/// Gets a directory from the app's [data directory](ProjectDirs::data_dir).
	pub fn get_data_dir(
		&self,
		purpose: &str,
		subdir: Option<PathBuf>,
	) -> crate::Result<(PathBuf, PathSource)> {
		self.get_or_create_app_dir(
			purpose,
			&DATA_FOLDER_ENV_VAR,
			|dirs| dirs.data_dir(),
			subdir,
		)
	}
}

impl Default for AppDirs {
	/// Constructs a set of project directories with [`CARGO_PKG_NAME`].
	fn default() -> Self {
		Self::new(&CARGO_PKG_NAME)
	}
}

impl Deref for AppDirs {
	type Target = Option<ProjectDirs>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for AppDirs {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

/// Initializes directories that are used in Terminal Arcade.
pub fn init_project_dirs(app_dirs: &AppDirs) -> crate::Result<()> {
	tracing::info!("initializing project dirs");
	app_dirs.get_or_create_app_dir(
		"config",
		&CONFIG_FOLDER_ENV_VAR,
		|dirs| dirs.config_dir(),
		None,
	)?;
	app_dirs.get_or_create_app_dir(
		"data",
		&DATA_FOLDER_ENV_VAR,
		|dirs| dirs.data_dir(),
		None,
	)?;
	Ok(())
}
