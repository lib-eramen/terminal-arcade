//! A struct that keeps track of scroll position. This is intended to simplify
//! the process of building a UI that has a number of navigable,
//! indexed elements

use std::cmp::min;

use rand::Rng;

/// Keeps track of scroll position.
/// This does not handle changing list lengths.
#[derive(Debug)]
pub struct ScrollTracker {
	/// The current position selected on the scroll.
	pub selected: Option<u64>,

	/// The starting index of the scroll.
	pub start: u64,

	/// The ending index of the scroll.
	pub end: u64,

	/// The range of the scroll list that is displayed.
	pub range: Option<u64>,

	/// The length of the scroll list.
	pub length: u64,
}

impl ScrollTracker {
	/// Creates a new unselected list that starts at 0.
	#[must_use]
	pub fn new(length: u64, range: Option<u64>) -> Self {
		Self {
			selected: None,
			start: 0,
			end: length - 1,
			range: Some(min(range.unwrap_or(length), length)),
			length,
		}
	}

	/// Returns if something has been selected in the scroll tracker.
	#[must_use]
	pub fn is_selected(&self) -> bool {
		self.selected.is_some()
	}

	/// Scrolls the list up.
	pub fn scroll_up(&mut self) {
		if self.selected.is_none() {
			if self.length > 0 {
				self.selected = Some(0);
			}
			return;
		}
		let selected = self.selected.unwrap();
		if selected == 0 {
			let location = self.length - 1;
			if let Some(range) = self.range {
				self.start = location - (range - 1);
			}
			self.selected = Some(self.length - 1);
		} else {
			self.selected = Some(selected - 1);
			if self.range.is_some() && selected == self.start {
				self.start =
					if selected < self.range.unwrap() { 0 } else { selected - self.range.unwrap() }
			}
		}
	}

	/// Scrolls the list down.
	pub fn scroll_down(&mut self) {
		if self.selected.is_none() {
			if self.length > 0 {
				self.selected = Some(0);
			}
			return;
		}
		let selected = self.selected.unwrap();
		if selected == self.length - 1 {
			self.start = 0;
			self.selected = Some(0);
		} else {
			self.selected = Some(selected + 1);
			if self.range.is_some() && selected == self.start + self.range.unwrap() - 1 {
				self.start = min(self.start + self.range.unwrap(), self.end);
			}
		}
	}

	/// Scrolls to a random spot in the scroll tracker.
	pub fn scroll_to_random(&mut self) {
		let mut rng = rand::thread_rng();
		self.start = rng.gen_range(0..self.length);
		self.selected = Some(self.start);
		self.end = std::cmp::min(self.end + self.range.unwrap_or(0), self.length - 1);
	}

	/// Sets a new length of the scroll tracker, also resetting the selected
	/// index.
	pub fn set_length(&mut self, new_length: u64) {
		self.length = new_length;
		self.start = 0;
		self.end = if new_length == 0 { 0 } else { new_length - 1 };
		self.selected = None;
	}

	/// Sets the range of the scroll tracker. Note that the function will panic
	/// if the range is outside of the specified size.
	pub fn set_range(&mut self, new_range: u64) {
		self.range = Some(min(self.length, new_range));
	}
}
