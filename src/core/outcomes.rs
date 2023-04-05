//! Functions for painless [Err]or handling through the [Outcome] type.

/// The common type of result used throughout Terminal Arcade.
/// The name `Outcome` was chosen because the code author had had enough
/// scrolling through suggestion lists for `Result`s.
pub type Outcome<T> = anyhow::Result<T>;

/// Converts any [Result]s into an [`Outcome`], with a mapper function.
pub fn out_comes<T, E, M, R>(result: Result<T, E>, mapper: M) -> Outcome<R>
where
	E: std::error::Error + Send + Sync + 'static,
	M: FnOnce(T) -> R, {
	result.map(mapper).map_err(anyhow::Error::new)
}

/// Converts any [Result]s to an [`Outcome`]`<()>`.
pub fn out_comes_unit<T, E>(result: Result<T, E>) -> Outcome<()>
where
	E: std::error::Error + Send + Sync + 'static, {
	out_comes(result, |_| ())
}

/// Converts any [Result]s to [`Outcome`]s, preserving the [`Ok`] value.
pub fn out_comes_preserve<T, E>(result: Result<T, E>) -> Outcome<T>
where
	E: std::error::Error + Send + Sync + 'static, {
	out_comes(result, |ok| ok)
}
