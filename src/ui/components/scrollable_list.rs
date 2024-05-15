//! A scrollable list. See [`ScrollableList`] for the struct this module
//! exports.

use std::{
	cmp::min,
	fmt::Display,
	time::Duration,
};

use derive_new::new;
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
	},
	prelude::{
		Buffer,
		Rect,
	},
	style::Modifier,
	widgets::{
		Paragraph,
		StatefulWidget,
		Widget,
	},
	Frame,
};

use crate::ui::components::{
	flicker_counter::FlickerCounter,
	presets::{
		highlight_block,
		titled_ui_block,
		HIGHLIGHTED,
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
///
/// This API exposes the underlying [scroll tracker](ScrollTracker) for access
/// to its API as well, containing functionality for changing the size of the
/// displayed list as well as scrolling.
#[derive(Clone)]
#[must_use]
pub struct ScrollableList<D: ToString + Clone> {
	/// Items to be displayed.
	items: Vec<ListItem<D>>,

	/// Scroll tracker for this list.
	scroll_tracker: ScrollTracker,

	/// Maximum number of lines allowed for one list item.
	max_item_lines: u16,

	/// Direction this list goes in.
	direction: Direction,

	/// Text alignment on the item blocks.
	text_alignment: Alignment,

	/// Margins for the list, vertically then horizontally.
	margins: (u16, u16),

	/// Flicker counter for the list.
	flicker_counter: FlickerCounter,
}

impl<D: ToString + Clone> ScrollableList<D> {
	/// Create a new scrollable list.
	pub fn new(
		items: Vec<ListItem<D>>,
		display_count: Option<usize>,
		max_item_lines: u16,
		direction: Direction,
		text_alignment: Alignment,
		custom_margins: Option<(u16, u16)>,
		custom_flicker_duration: Option<Duration>,
	) -> Self {
		let scroll_tracker = ScrollTracker::new(items.len(), display_count);
		let margins = custom_margins.unwrap_or((1, 2));
		let flicker_counter =
			FlickerCounter::new(custom_flicker_duration.unwrap_or(Duration::from_secs_f32(0.5)));
		Self {
			items,
			scroll_tracker,
			max_item_lines,
			direction,
			text_alignment,
			margins,
			flicker_counter,
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
	pub fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
		let chunks = self.get_layout().split(area);
		for (position, index) in self.scroll_tracker.get_displayed_range().enumerate() {
			self.render_raw_item(frame, chunks[position], index, None);
		}
	}

	/// Renders this list with a custom closure to process the raw string
	/// displayed on the list.
	pub fn render_processed<P>(&mut self, frame: &mut Frame<'_>, area: Rect, processor: P)
	where
		P: Fn(&ListItem<D>) -> Paragraph<'_>,
	{
		let chunks = self.get_layout().split(area);
		let items = self.items.clone();
		for (position, index) in self.scroll_tracker.get_displayed_range().enumerate() {
			let item = items.get(index).unwrap_or_else(|| {
				panic!(
					"list length is {} but tried to index at {index}",
					items.len()
				)
			});
			self.render_item_processed(frame, chunks[position], item, index, &processor);
		}
	}

	/// Renders one item of this list.
	///
	/// # Panics
	///
	/// This function panics when the index is outside of the list's items.
	fn render_raw_item(
		&mut self,
		frame: &mut Frame<'_>,
		area: Rect,
		index: usize,
		custom_paragraph: Option<Paragraph<'_>>,
	) {
		let item = self.items.get(index).unwrap_or_else(|| {
			panic!(
				"list length is {} but tried to index at {index}",
				self.items.len()
			)
		});
		let mut item_block = titled_ui_block(format!(
			"{}{}",
			index + 1,
			item.name.as_ref().map_or(String::new(), |s| format!(" ─ {s}"))
		))
		.title_alignment(self.text_alignment);

		if self.get_selected().map_or(false, |(selected_index, _)| index == selected_index) {
			let mut style = HIGHLIGHTED;
			if self.flicker_counter.is_off() {
				style = style.add_modifier(Modifier::DIM);
			}
			item_block = highlight_block(item_block).style(style);
		}
		let paragraph =
			custom_paragraph.unwrap_or_else(|| Paragraph::new(item.get_displayed_data()));
		let item_paragraph = paragraph.alignment(self.text_alignment).block(item_block);
		frame.render_widget(item_paragraph, area);
		self.flicker_counter.update();
	}

	/// Renders one item of this list after being passed through a processor
	/// function.
	///
	/// # Panics
	///
	/// This function panics when the index is outside of the list's items.
	fn render_item_processed<P>(
		&mut self,
		frame: &mut Frame<'_>,
		area: Rect,
		item: &ListItem<D>,
		index: usize,
		processor: P,
	) where
		P: Fn(&ListItem<D>) -> Paragraph<'_>,
	{
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

	/// Returns

	/// Updates items this list displays as well as the length of the underlying
	/// scroll tracker.
	pub fn update_items(&mut self, items: Vec<ListItem<D>>) {
		self.items = items;
		self.scroll_tracker.set_length(self.items.len());
	}

	/// Scrolls the list forward, or back to start if the list is at the end.
	pub fn scroll_forward(&mut self) {
		self.scroll_tracker.scroll_forward();
		self.flicker_counter.reset();
	}

	/// Scrolls the list forward, or back to start if the list is at the end.
	pub fn scroll_backward(&mut self) {
		self.scroll_tracker.scroll_backward();
		self.flicker_counter.reset();
	}

	/// Scrolls the list to a random position.
	pub fn scroll_to_random(&mut self) {
		self.scroll_tracker.scroll_to_random();
		self.flicker_counter.reset();
	}

	/// Gets the list's display count.
	#[must_use]
	pub fn get_display_count(&self) -> Option<usize> {
		self.scroll_tracker.display_count
	}

	/// Sets the list's display count.
	pub fn set_display_count(&mut self, display_count: usize) {
		assert!(display_count > 0);
		self.scroll_tracker.set_display_count(display_count);
	}
}
