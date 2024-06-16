//! A table of controls and what they do. See [`ControlsTable`] for more.

use std::fmt::Display;

use bitflags::bitflags;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use derive_builder::Builder;
use derive_new::new;
use indexmap::IndexMap;
use ratatui::{
	layout::{Constraint, Direction, Layout},
	prelude::{Buffer, Rect},
	style::{Modifier, Style, Stylize},
	text::Text,
	widgets::{Cell, Row, StatefulWidget, Table, TableState},
};

use crate::ui::{
	components::presets::HIGHLIGHTED,
	widgets::{utils::scroll_tracker::ScrollTracker, Widget, WidgetFocus, WidgetState},
};

/// The main key of a key combination, versus a modifier.
#[derive(Hash, PartialEq, Eq)]
pub enum KeyControl {
	/// A typable character on the keyboard.
	Char(char),

	/// Function keys.
	F(u8),

	/// Custom control entry.
	Custom(String),
}

impl KeyControl {
	/// Creates a new custom key control.
	pub fn new_custom<S: ToString>(s: S) -> Self {
		Self::Custom(s.to_string())
	}
}

impl Clone for KeyControl {
	fn clone(&self) -> Self {
		match self {
			KeyControl::Char(c) => KeyControl::Char(*c),
			KeyControl::F(n) => KeyControl::F(*n),
			KeyControl::Custom(s) => KeyControl::Custom(s.clone()),
		}
	}
}

impl Display for KeyControl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				KeyControl::Char(c) => c.to_string().to_uppercase(),
				KeyControl::F(n) => format!("F{n}"),
				KeyControl::Custom(ref s) => s.to_string(),
			}
		)
	}
}

/// A key combination that does something, a.k.a. a control.
#[derive(Clone, Hash, PartialEq, Eq, new)]
pub struct Control {
	/// Modifier key names.
	modifiers: Option<Vec<String>>,

	/// The main key.
	control: KeyControl,
}

impl Display for Control {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut keys = self.modifiers.clone().unwrap_or_else(|| Vec::new());
		keys.push(self.control.to_string());
		let result = keys.into_iter().map(|key| format!("[{key}]")).collect::<Vec<_>>().join(" ");
		write!(f, "{result}")
	}
}

/// A table of [Control]s, each mapped to a function/usage described in a text
/// entry. The table also handles merging controls with like key combinations.
///
/// One way to instantiate this struct would be to create a new [Default]
/// instance before [`Self::register`]ing entries.
#[derive(Clone, Default)]
pub struct ControlsEntries(pub IndexMap<Control, Vec<String>>);

impl<'a> ControlsEntries {
	/// Creates a new controls entries table.
	pub fn new<E>(entries: E) -> Self
	where
		E: IntoIterator<Item = (Control, Vec<String>)>,
	{
		Self(IndexMap::from_iter(entries))
	}

	/// Adds an entry into the controls entries table. This is a fluent setter
	/// method.
	pub fn add<S: ToString>(mut self, control: Control, entry: S) -> Self {
		self.register(control, entry.to_string());
		self
	}

	/// Adds an entry of multiple functions in to the controls entries table.
	/// This is a fluent setter method.
	pub fn add_multi(mut self, control: Control, entries: Vec<String>) -> Self {
		for entry in entries {
			self = self.add(control.clone(), entry);
		}
		self
	}

	/// Registers an entry, merging into an exact control if it exists.
	fn register(&mut self, control: Control, entry: String) {
		self.0
			.entry(control.clone())
			.and_modify(|entries| entries.push(entry.clone()))
			.or_insert(vec![entry]);
	}

	/// Creates a new controls entries table, with reference to other controls
	/// table to be [`Self::merge`]d.
	pub fn with_others<E>(entries: E, others: &[&Self]) -> Self
	where
		E: IntoIterator<Item = (Control, Vec<String>)>,
	{
		let mut result = Self::new(entries);
		for other in others {
			result.merge(other);
		}
		result
	}

	/// Merges another controls entries table with this control table, enabling
	/// enabling
	pub fn merge(&mut self, other: &Self) {
		for (control, entry) in &other.0 {
			self.0
				.entry(control.clone())
				.and_modify(|entries| entries.append(&mut entry.clone()))
				.or_insert(entry.clone());
		}
	}

	/// Gets the longest control string's length.
	pub fn get_longest_control_str_len(&self) -> Option<usize> {
		self.0.iter().map(|(control, _)| control.to_string().len()).fold(None, |acc, item| {
			if item > acc.unwrap_or(0) {
				Some(item)
			} else {
				acc
			}
		})
	}

	/// Gets the longest entry string's length.
	pub fn get_longest_entry_str_len(&self, index: usize) -> Option<usize> {
		self.0.get_index(index)?.1.iter().map(String::len).fold(None, |acc, item| {
			if item > acc.unwrap_or(0) {
				Some(item)
			} else {
				acc
			}
		})
	}
}

/// A table of [Control]s, each mapped to a function/usage described in text.
#[derive(Clone)]
pub struct ControlsTable {
	/// Controls entries to be displayed.
	controls_entries: ControlsEntries,

	/// Scroll tracker for the table.
	scroll_tracker: ScrollTracker,
}

impl Widget for ControlsTable {
	/// Returns this widget's initial state.
	fn initial_state(&self) -> WidgetState {
		WidgetState::new(
			WidgetFocus::Unfocused,
			ControlsEntries::default().add(
				Control::new(None, KeyControl::new_custom("[↑ ↓]")),
				"Navigate this controls list",
			),
		)
	}

	fn handle_event(&mut self, event: &Event) -> anyhow::Result<()> {
		// TODO: Should it also handle HJKL and WASD?
		if let Event::Key(KeyEvent {
			code, modifiers, ..
		}) = event
		{
			if modifiers.is_empty() {
				match code {
					KeyCode::Up => self.scroll_tracker.scroll_backward(),
					KeyCode::Down => self.scroll_tracker.scroll_forward(),
					_ => {},
				}
			}
		}
		Ok(())
	}

	fn render_ui(&self, frame: &mut ratatui::Frame<'_>, area: Rect, state: &WidgetState) {
		// TODO: Use state to make selected option flicker

		let mut table_state = TableState::from(self.scroll_tracker);
		let controls_entries = &self.controls_entries;

		let header = ["Control", "Function"]
			.into_iter()
			.map(Cell::from)
			.collect::<Row<'_>>()
			.style(HIGHLIGHTED.add_modifier(Modifier::UNDERLINED))
			.height(1);
		// TODO: (Util function?) Alternating colors for alternating rows.
		let entry_rows = {
			let mut rows = controls_entries
				.0
				.iter()
				.map(|(control, entries)| {
					let entry_length = entries.len();
					let entry_height = entry_length
						.try_into()
						.expect(format!("Too many lines: {entry_length} > {}", u16::MAX).as_str());

					Row::new([
						Cell::new(control.to_string()).italic(),
						Cell::new(entries.join("\n")),
					])
					.height(entry_height)
				})
				.collect::<Vec<_>>();
			rows.insert(0, header);
			rows
		};

		let table_widths = [Constraint::Length(
			self.controls_entries.get_longest_control_str_len().unwrap_or(0) as u16,
		)];
		let table = Table::new(entry_rows, table_widths);
		frame.render_stateful_widget(table, area, &mut table_state);
	}
}
