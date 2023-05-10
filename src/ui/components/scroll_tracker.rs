//! A struct that keeps track of scroll position. This is intended to simplify
//! the process of building a UI that has a constant number of navigable,
//! indexed elements

/// Keeps track of scroll position.
/// This does not handle changing list lengths.
pub struct ScrollTracker {
	/// The current position selected on the scroll.
	pub selected: Option<u64>,

	/// The starting index of the scroll.
	pub start: u64,

	/// The ending index of the scroll.
	pub end: u64,
}

impl ScrollTracker {
	/// Creates a new unselected list that starts at 0.
	#[must_use]
	pub fn new(end: u64) -> Self {
		Self {
			selected: None,
			start: 0,
			end,
		}
	}

	/// Scrolls the list up.
	pub fn scroll_up(&mut self) {
		if let Some(index) = self.selected {
			self.selected = Some(if index == 0 { self.end } else { index - 1 });
		} else {
			self.selected = Some(0);
		}
	}

	/// Scrolls the list down.
	pub fn scroll_down(&mut self) {
		if let Some(index) = self.selected {
			self.selected = Some(if index == self.end { 0 } else { index + 1 });
		} else {
			self.selected = Some(0);
		}
	}
}
