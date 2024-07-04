fn main() -> anyhow::Result<()> {
	vergen::EmitBuilder::builder().all_build().all_git().emit()?;
	Ok(())
}
