use std::{
	fs::*,
	path::{
		Path,
		PathBuf,
	},
};

use vergen_gix::*;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn init_vergen() -> Result<()> {
	Emitter::default()
		.add_instructions(&BuildBuilder::all_build()?)?
		.add_instructions(&CargoBuilder::all_cargo()?)?
		.add_instructions(&GixBuilder::all_git()?)?
		.add_instructions(&RustcBuilder::all_rustc()?)?
		.add_instructions(&SysinfoBuilder::all_sysinfo()?)?
		.emit()?;
	Ok(())
}

// https://stackoverflow.com/a/65192210/26515777
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
	create_dir_all(&dst)?;
	for entry in read_dir(src)? {
		let entry = entry?;
		let ty = entry.file_type()?;
		if ty.is_dir() {
			copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
		} else {
			copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
		}
	}
	Ok(())
}

fn init_assets() -> Result<()> {
	let dirs = directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
		.expect("unable to retrieve project dirs");
	let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let local_assets_dir = manifest_dir.join("assets");
	let asset_dir = dirs.data_dir().join(".assets");
	let _ = remove_dir_all(asset_dir.clone());
	copy_dir_all(local_assets_dir, asset_dir)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	init_vergen()?;
	init_assets()?;
	Ok(())
}
