//! [`Event`]s sent throughout the app. An event can be low-level and come from
//! the terminal (usually via [`Event::Tui`]) or more abstract and resemble a
//! command, coming from [`Screen`](crate::ui::screen::Screen)s.

use crate::app::AppEvent;

/// Events sent throughout the app.
/// Each variant should be a tuple struct containing a subset of events
/// sent from a particular source.
#[derive(Debug, Clone, Hash)]
pub enum Event {
	/// General events for the [app](crate::app::App) to handle.
	App(AppEvent),
}

macro_rules! impl_action_from_event {
	($source:ident, $variant:ident) => {
		impl From<$source> for Event {
			fn from(value: $source) -> Self {
				Self::$variant(value)
			}
		}
	};
}

impl_action_from_event!(AppEvent, App);
