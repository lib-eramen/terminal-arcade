//! A module containing the [Screen] trait, a trait needed to, basically, do
//! everything on the terminal in Terminal Arcade. See the [Screen] trait to get
//! started. It also contains the various screens that the game uses to present
//! itself on the terminal.

pub mod color_scheme;
pub mod components;
pub mod screens;
pub mod util;
pub mod widgets;

pub use screens::*;
