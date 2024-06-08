//! A table of controls and what they do. See [`ControlsTable`] for more.

use std::fmt::Display;

use bitflags::bitflags;
use derive_builder::Builder;
use derive_new::new;
use indexmap::IndexMap;

/// The main key of a key combination, versus a modifier.
#[derive(Hash, PartialEq, Eq)]
pub enum KeyControl {
	/// A typable character on the keyboard.
	Char(char),

	/// Function keys.
	F(u8),

	/// Custom entry.
	Custom(String),
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
		write!(f, "{}", match self {
			KeyControl::Char(c) => c.to_string().to_uppercase(),
			KeyControl::F(n) => format!("F{n}"),
			KeyControl::Custom(ref s) => s.to_string(),
		})
	}
}

/// A key combination that does something, a.k.a. a control.
#[derive(Clone, Hash, PartialEq, Eq, new)]
pub struct Control {
	/// Modifier key names.
	modifiers: Vec<String>,

	/// The main key.
	control: KeyControl,
}

impl Display for Control {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut keys = self.modifiers.clone();
		keys.push(self.control.to_string());
		let result = keys.into_iter().map(|key| format!("[{key}]")).collect::<Vec<_>>().join(" ");
		write!(f, "{result}")
	}
}

/// A table of [Control]s, each mapped to a function/usage described in text.
/// The table also handles merging controls with like key combinations.
#[derive(Clone)]
pub struct ControlsTable(IndexMap<Control, Vec<String>>);

impl ControlsTable {
	/// Creates a controls table.
	pub fn new<T: IntoIterator<Item = (Control, Vec<String>)>>(iterable: T) -> Self {
		Self(IndexMap::from_iter(iterable))
	}

	/// Creates a controls table, with reference to other controls table to be
	/// [`Self::merge`]d.
	pub fn with_others<T: IntoIterator<Item = (Control, Vec<String>)>>(
		iterable: T,
		others: &[&Self],
	) -> Self {
		let mut result = Self::new(iterable);
		for other in others {
			result.merge(other);
		}
		result
	}

	/// Merges another controls table with this control table, enabling enabling
	pub fn merge(&mut self, other: &Self) {
		for (control, entry) in &other.0 {
			self.0
				.entry(control.clone())
				.and_modify(|entries| entries.append(&mut entry.clone()))
				.or_insert(entry.clone());
		}
	}
}
