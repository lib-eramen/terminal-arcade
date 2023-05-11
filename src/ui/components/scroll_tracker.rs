//! A struct that keeps track of scroll position. This is intended to simplify
//! the process of building a UI that has a number of navigable,
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

	/// The range of the scroll list that is displayed.
	pub range: Option<u64>,
}

impl ScrollTracker {
	/// Gets the length of the tracker
	#[must_use]
	pub fn length(&self) -> u64 {
		self.end - self.start + 1
	}
	/// Creates a new unselected list that starts at 0.
	#[must_use]
	pub fn new(end: u64, range: Option<u64>) -> Self {
		Self {
			selected: None,
			start: 0,
			end, range
		}
	}

	/// Scrolls the list up.
	pub fn scroll_up(&mut self) {
		let length = self.length();
		if self.selected.is_none() {
			if length > 0 {
				self.selected = Some(0);
			}
			return;
		}
		let selected = self.selected.unwrap();
		if selected == 0 {
			self.start = length - 1;
			self.selected = Some(length - 1);
		} else {
			self.selected = Some(selected - 1);
			if selected == self.start && self.range.is_some() {
				self.start = if selected < self.range.unwrap() {
					0
				} else {
					selected - self.range.unwrap()
				}
			}
		}
	}

	/// Scrolls the list down.
	pub fn scroll_down(&mut self) {
		let length = self.length();
		if self.selected.is_none() {
			if length > 0 {
				self.selected = Some(0);
			}
			return;
		}
		let selected = self.selected.unwrap();
		if selected == length - 1 {
			self.start = 0;
			self.selected = Some(0);
		} else {
			self.selected = Some(selected + 1);
			if self.range.is_some() && selected == self.range.unwrap() {
				self.start = if selected < self.range.unwrap() {
					0
				} else {
					selected - self.range.unwrap()
				}
			}
		}
	}
}
