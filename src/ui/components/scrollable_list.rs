//! A scrollable list. See [`ScrollableList`] for the struct this module
//! exports.

use std::{
	cmp::min,
	fmt::Display,
};

use derive_new::new;
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
	},
	prelude::Rect,
	widgets::Paragraph,
	Frame,
};

use crate::ui::components::{
	presets::{
		highlight_block,
		titled_ui_block,
	},
	scroll_tracker::ScrollTracker,
};

/// A list item to be displayed.
#[derive(Clone, new)]
#[must_use]
pub struct ListItem<D: ToString> {
	/// Title for a block that wraps the item when displayed.
	pub name: Option<String>,

	/// Data associated with this item.
	pub data: D,

	/// Content to be displayed on screen instead of the data.
	pub displayed_content: Option<String>,
}

impl<D: ToString> ListItem<D> {
	/// Returns data to be displayed for this list item.
	pub fn get_displayed_data(&self) -> String {
		self.displayed_content.clone().unwrap_or(self.data.to_string())
	}
}

/// A scrollable list that highlights the chosen element, with adjustable view
/// range.
/// This API exposes the underlying scroll tracker for access to its API as
/// well, containing functionality for changing the size of the displayed list
/// as well as scrolling.
#[derive(Clone)]
#[must_use]
pub struct ScrollableList<D: ToString> {
	/// Items to be displayed.
	pub items: Vec<ListItem<D>>,

	/// Scroll tracker for this list.
	pub scroll_tracker: ScrollTracker,

	/// Maximum number of lines allowed for one list item.
	pub max_item_lines: u16,

	/// Direction this list goes in.
	pub direction: Direction,

	/// Text alignment on the item blocks.
	pub text_alignment: Alignment,

	/// Margins for the list, vertically then horizontally.
	pub margins: (u16, u16),
}

impl<D: ToString> ScrollableList<D> {
	/// Create a new scrollable list.
	pub fn new(
		items: Vec<ListItem<D>>,
		display_count: Option<usize>,
		max_item_lines: u16,
		direction: Direction,
		text_alignment: Alignment,
		custom_margins: Option<(u16, u16)>,
	) -> Self {
		let tracker = ScrollTracker::new(items.len(), display_count);
		let margins = custom_margins.unwrap_or((1, 2));
		Self {
			items,
			scroll_tracker: tracker,
			max_item_lines,
			direction,
			text_alignment,
			margins,
		}
	}

	/// Returns the selected item in the list.
	/// The first element in the returned tuple is the index where the element
	/// was found.
	#[must_use]
	pub fn get_selected(&self) -> Option<(usize, &ListItem<D>)> {
		let selected_index = self.scroll_tracker.selected?;
		let item = self.items.get(selected_index)?;
		Some((selected_index, item))
	}

	/// Renders this list.
	pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
		let chunks = self.get_layout().split(area);
		for (position, index) in self.scroll_tracker.get_displayed_range().enumerate() {
			self.render_raw_item(frame, chunks[position], index, None);
		}
	}

	/// Renders this list with a custom closure to process the raw string
	/// displayed on the list.
	pub fn render_processed<P>(&self, frame: &mut Frame<'_>, area: Rect, processor: P)
	where
		P: Fn(&ListItem<D>) -> Paragraph<'_>,
	{
		let chunks = self.get_layout().split(area);
		for (position, index) in self.scroll_tracker.get_displayed_range().enumerate() {
			self.render_item_processed(frame, chunks[position], index, &processor);
		}
	}

	/// Renders one item of this list.
	///
	/// # Panics
	///
	/// This function panics when the index is outside of the list's items.
	fn render_raw_item(
		&self,
		frame: &mut Frame<'_>,
		area: Rect,
		index: usize,
		custom_paragraph: Option<Paragraph<'_>>,
	) {
		let item = self.items.get(index).expect("Index outside of list's range.");
		let mut item_block = titled_ui_block(format!(
			"{}{}",
			index + 1,
			item.name.as_ref().map_or(String::new(), |s| format!(" ─ {s}"))
		))
		.title_alignment(self.text_alignment);
		if self.scroll_tracker.selected.map_or(false, |selected| selected == index) {
			item_block = highlight_block(item_block);
		}
		let paragraph =
			custom_paragraph.unwrap_or_else(|| Paragraph::new(item.get_displayed_data()));
		let item_paragraph = paragraph.alignment(self.text_alignment).block(item_block);
		frame.render_widget(item_paragraph, area);
	}

	/// Renders one item of this list after being passed through a processor
	/// function.
	///
	/// # Panics
	///
	/// This function panics when the index is outside of the list's items.
	fn render_item_processed<P>(
		&self,
		frame: &mut Frame<'_>,
		area: Rect,
		index: usize,
		processor: P,
	) where
		P: Fn(&ListItem<D>) -> Paragraph<'_>,
	{
		let item = self.items.get(index).expect("Index outside of list's range.");
		let paragraph = processor(item);
		self.render_raw_item(frame, area, index, Some(paragraph));
	}

	/// Returns the layout for this list. Put simply, the layout is only a list
	/// of vertically-scrolling boxes.
	#[must_use]
	pub fn get_layout(&self) -> Layout {
		let mut constraints = vec![Constraint::Max(self.max_item_lines + 2); self.items.len()];
		constraints.push(Constraint::Max(0));

		Layout::default()
			.direction(self.direction)
			.vertical_margin(self.margins.0)
			.horizontal_margin(self.margins.1)
			.constraints(constraints)
	}

	/// Updates items this list displays as well as the length of the underlying
	/// scroll tracker.
	pub fn update_items(&mut self, items: Vec<ListItem<D>>) {
		self.items = items;
		self.scroll_tracker.set_length(self.items.len());
	}
}
