//! A module containing the authors of the games included in Terminal Arcade.
//!
//! Hey future contributors with huge desires for recognition!
//!
//! Wait, where was I going?
//!
//! Oh right. Add your name here if you make a game! For each author,
//! add a `const` variable with a distinctive-enough identifier (no SCREAMING_SNAKE_CASE enforced!),
//! and put your name in! You might want to also put your Git identifier/
//! GitHub username in the doc comment.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/// Developer Ramen <ramendev2009@gmail.com>, @developer-ramen
pub const ramendev: &str = "@developer-ramen";

/// Returns a list of all of the game authors.
pub fn all_authors() -> Vec<&'static str> {
	vec![ramendev]
}
