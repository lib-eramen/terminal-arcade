//! Official color scheme used throughout the application.
//! The author took a lot of time to describe how these colors look like in
//! plain language. Please use it, and thank you!

use ratatui::style::Color;

/// Type alias for Color (3-long [u8] array).
pub type RGB = [u8; 3];

/// Soft, warm yellow evoking the feeling of the sun.
pub static SUNGLOW: Color = get_color([253, 202, 64]);

/// Pale white with a soft touch of blue.
pub static GHOST_WHITE: Color = get_color([254, 249, 255]);

/// Very washed out purple thistle color.
pub static THISTLE: Color = get_color([212, 193, 236]);

/// Light indigo color, akin to a purplish sky blue.
pub static TROPICAL_INDIGO: Color = get_color([159, 159, 237]);

/// Blue with lots of purple.
pub static MEDIUM_SLATE_BLUE: Color = get_color([115, 108, 237]);

/// Bright, warm violet.
pub static FRENCH_VIOLET: Color = get_color([127, 44, 203]);

/// Creates a [Color] from a given [Color] array.
#[must_use]
pub const fn get_color(rgb: RGB) -> Color {
	Color::Rgb(rgb[0], rgb[1], rgb[2])
}
