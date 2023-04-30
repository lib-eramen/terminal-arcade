//! A module containing the [Screen] trait, a trait needed to, basically, do
//! everything on the terminal in Terminal Arcade. See the [Screen] trait to get
//! started. It also contains the various screens that the game uses to present
//! itself on the terminal.

pub mod screens;
pub mod util;

/// The level of indentation to be used for printing. This is 8 spaces.
/// This static variable is intended to be replaced with a configurable
/// indentation option. TODO: Configuration option for indent.
pub static INDENT: &str = r#"        "#;

pub use screens::*;
