//! A text input bar. See [`TextInputBar`] for the struct this module exports.

use crate::ui::widgets::util::flicker_counter::FlickerCounter;

/// A text input field, navigable with entry by a keyboard-controlled cursor.
#[derive(Clone)]
#[must_use]
pub struct TextInputField {
	/// Text currently in this field.
	text: Option<String>,

	/// Placeholder text in the field when it is empty.
	placeholder: Option<String>,

	/// Maximum number of characters allowed in the field.
	max_len: usize,

	/// Flicker counter for the list.
	flicker_counter: FlickerCounter,
}
