//! Banner showing the Terminal Arcade ASCII art logo.

use lazy_static::lazy_static;

lazy_static! {
	static ref BANNER: String = std::fs::read_to_string("path");
}