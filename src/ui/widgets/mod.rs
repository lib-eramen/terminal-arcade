//! Module for widgets to display units of data. See [Widget] to get started.

use crossterm::event::Event;
use derive_new::new;
use enum_dispatch::enum_dispatch;
use ratatui::{
	prelude::{
		Buffer,
		Rect,
	},
	widgets::StatefulWidget,
	Frame,
};

use crate::ui::widgets::utils::controls_table::ControlsEntries;

pub mod scrollable_list;
pub mod utils;

/// No state.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Stateless;

/// Focus state of a [Widget].
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum WidgetFocus {
	Focused,
	Unfocused,
	Unfocusable,
}

/// Widgets' common state.
#[derive(Clone, new)]
pub struct WidgetState {
	/// This widget's focus state.
	pub focus: WidgetFocus,

	/// Controls of the widget.
	pub controls: ControlsEntries,
}

/// A widget, helpful to display specific formats of data and handle.
/// This trait does not follow conventions similar to what [ratatui] does,
/// separating the state from the rendering and needing to be created
/// every time it is rendered. Due to language limitations,
pub trait Widget {
	/// Returns this widget's initial state.
	fn initial_state(&self) -> WidgetState;

	/// Handles an event.
	/// Refer to [`crate::ui::screens::Screen::handle_event`] for events that
	/// are intercepted by the overlying screen that manages this widget.
	fn handle_event(&mut self, event: &Event) -> anyhow::Result<()>;

	/// Renders this widget's UI.
	fn render_ui(&self, frame: &mut Frame<'_>, area: Rect, state: &WidgetState);

	/// Updates this struct on a frame-by-frame basis.
	fn update(&mut self, _state: &mut WidgetState) {}
}
