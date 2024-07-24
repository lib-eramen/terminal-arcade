fn main() -> Result<(), Box<dyn std::error::Error>> {
	use vergen_gix::*;
	Emitter::default()
		.add_instructions(&BuildBuilder::all_build()?)?
		.add_instructions(&CargoBuilder::all_cargo()?)?
		.add_instructions(&GixBuilder::all_git()?)?
		.add_instructions(&RustcBuilder::all_rustc()?)?
		.add_instructions(&SysinfoBuilder::all_sysinfo()?)?
		.emit()?;
	Ok(())
}
