//! [`Event`]s sent throughout the app. An event can be low-level and come from
//! the terminal (usually via [`Event::Tui`]) or more abstract and resemble a
//! command, coming from [`Screen`](crate::ui::screen::Screen)s.

pub mod app;
pub mod input;
pub mod screen;
pub mod tui;
pub mod ui;
pub mod util;

pub use app::AppEvent;
pub use input::InputEvent;
pub use screen::ScreenEvent;
pub use tui::TuiEvent;
pub use util::*;

/// Events sent throughout and handled by the [`App`](crate::app::App).
/// Each variant should be a tuple struct containing a subset of events
/// sent from a particular source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Event {
	/// General events for the [`App`](crate::app::App) to handle.
	App(AppEvent),

	/// Screen-manipulating events.
	Screen(ScreenEvent),

	/// Input events that gets passed down to screens.
	Input(InputEvent),
}

impl Event {
	/// Returns whether this event should be logged.
	pub fn should_be_logged(&self) -> bool {
		match self {
			Event::App(app_event) => app_event.should_be_logged(),
			Event::Input(_) | Event::Screen(_) => true,
		}
	}
}

/// Implements [`From`] for [`Event`], converting an underlying event type to a
/// variant name that holds a single value of that type.
macro_rules! impl_event_from_wrapped_variant {
	($source:ident, $variant:ident) => {
		impl From<$source> for Event {
			fn from(value: $source) -> Self {
				Self::$variant(value)
			}
		}
	};
}

/// Applies [`impl_event_from_wrapped_variant`] for multiple pairs of events and
/// variants.
macro_rules! impl_event_from_variants {
	($(($source:ident, $variant:ident)),+$(,)*) => {
		$(impl_event_from_wrapped_variant!($source, $variant);)+
	};
}

impl_event_from_variants! {
	(AppEvent, App),
	(InputEvent, Input),
	(ScreenEvent, Screen),
}
