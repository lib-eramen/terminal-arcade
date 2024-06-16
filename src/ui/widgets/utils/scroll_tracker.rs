//! A struct that keeps track of scroll position. This is intended to simplify
//! the process of building a UI that has a number of navigable,
//! indexed elements

use std::{cmp::min, ops::Range};

use anyhow::bail;
use rand::Rng;
use ratatui::widgets::TableState;

/// Keeps track of scroll position.
#[derive(Debug, Clone, Copy)]
pub struct ScrollTracker {
	/// The current position selected on the scroll.
	pub selected: Option<usize>,

	/// The starting index of the scroll.
	pub start: usize,

	/// The ending index of the scroll.
	pub end: usize,

	/// The range of the scroll list that is displayed.
	pub display_count: Option<usize>,

	/// The length of the scroll list.
	pub length: usize,
}

impl ScrollTracker {
	/// Creates a new unselected list that starts at 0.
	#[must_use]
	pub fn new(length: usize, range: Option<usize>) -> Self {
		Self {
			selected: None,
			start: 0,
			end: length - 1,
			display_count: Some(min(range.unwrap_or(length), length)),
			length,
		}
	}

	/// Returns if something has been selected in the scroll tracker.
	#[must_use]
	pub fn is_selected(&self) -> bool {
		self.selected.is_some()
	}

	/// Returns the range to be displayed, according to this tracker.
	#[must_use]
	pub fn get_displayed_range(&self) -> Range<usize> {
		let range_to_end = self.start..self.length;
		if let Some(count) = self.display_count {
			if self.start + count > self.length {
				range_to_end
			} else {
				self.start..(self.start + count)
			}
		} else {
			range_to_end
		}
	}

	/// Scrolls the list forward.
	pub fn scroll_forward(&mut self) {
		if self.selected.is_none() {
			if self.length > 0 {
				self.selected = Some(0);
			}
			return;
		}
		let selected = self.selected.unwrap();
		if selected == 0 {
			let location = self.length - 1;
			if let Some(range) = self.display_count {
				self.start = location - (range - 1);
			}
			self.selected = Some(self.length - 1);
		} else {
			self.selected = Some(selected - 1);
			if self.display_count.is_some() && selected == self.start {
				self.start = if selected < self.display_count.unwrap() {
					0
				} else {
					selected - self.display_count.unwrap()
				}
			}
		}
	}

	/// Scrolls the list backward.
	pub fn scroll_backward(&mut self) {
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
			if self.display_count.is_some()
				&& selected == self.start + self.display_count.unwrap() - 1
			{
				self.start = min(self.start + self.display_count.unwrap(), self.end);
			}
		}
	}

	/// Scrolls to a random spot in the scroll tracker.
	pub fn scroll_to_random(&mut self) {
		let mut rng = rand::thread_rng();
		self.start = rng.gen_range(0..self.length);
		self.selected = Some(self.start);
		self.end = std::cmp::min(self.end + self.display_count.unwrap_or(0), self.length - 1);
	}

	/// Sets a new length of the scroll tracker, also resetting the selected
	/// index. This function does not update the [`Self::display_count`]
	/// property.
	pub fn set_length(&mut self, new_length: usize) {
		self.length = new_length;
		self.start = 0;
		self.end = if new_length == 0 { 0 } else { new_length - 1 };
		self.selected = None;
	}

	/// Sets the display count of the scroll tracker.
	/// Note that if the new count value is larger than the tracker's length,
	/// the count will be set to the tracker's length instead.
	pub fn set_display_count(&mut self, new_range: usize) {
		self.display_count = Some(min(self.length, new_range));
	}
}

impl From<ScrollTracker> for TableState {
	fn from(value: ScrollTracker) -> Self {
		TableState::new().with_selected(value.selected)
	}
}
