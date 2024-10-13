//! Banner showing the Terminal Arcade ASCII art logo.

use std::path::PathBuf;

use crate::services::files::AppFiles;

/// Gets the banner logo.
fn get_logo_banner_text(files: &AppFiles) -> crate::Result<String> {
	Ok(std::fs::read_to_string(files.get_asset_path(
		PathBuf::from("banners").join("logo.txt"),
	)?)?)
}
