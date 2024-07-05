fn main() -> anyhow::Result<()> {
	vergen::EmitBuilder::builder()
		.all_build()
		.all_cargo()
		.all_git()
		.all_rustc()
		.all_sysinfo()
		.emit()?;
	Ok(())
}
