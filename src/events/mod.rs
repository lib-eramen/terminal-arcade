//! [`Event`]s sent throughout the app. An event can be low-level and come from
//! the terminal (usually via [`Event::Tui`]) or more abstract and resemble a
//! command, coming from [`Screen`](crate::ui::screen::Screen)s.

pub mod app;
pub mod tui;

pub use app::AppEvent;
use tracing::info;
pub use tui::TuiEvent;

use crate::events::tui::InputEvent;

/// Events sent throughout and handled by the [`App`](crate::app::App).
/// Each variant should be a tuple struct containing a subset of events
/// sent from a particular source.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
	/// General events for the [`App`](crate::app::App) to handle.
	App(AppEvent),
}

impl Event {
	/// Returns whether this event should be logged.
	pub fn should_be_logged(&self) -> bool {
		match self {
			Event::App(app_event) => app_event.should_be_logged(),
		}
	}
}

/// Implements [`From`] for [`Event`], converting an underlying event type to a
/// variant name that holds a single value of that type.
macro_rules! impl_event_from_variant {
	($source:ident, $variant:ident) => {
		impl From<$source> for Event {
			fn from(value: $source) -> Self {
				Self::$variant(value)
			}
		}
	};
}

impl_event_from_variant!(AppEvent, App);

/// A [`Tui`]-to-[`App`]preprocessor to transform raw [`TuiEvent`]s sent by the
/// [`Tui`] into [`AppEvent`] that the [`App`] can use.
///
/// [`Tui`]: crate::tui::Tui
/// [`App`]: crate::app::App
#[derive(Debug, Default)]
pub struct TuiAppMiddleman {
	/// Buffer for accumulating input events per tick.
	input_buffer: Vec<InputEvent>,
}

impl TuiAppMiddleman {
	#[allow(clippy::unwrap_used, reason = "infallible")]
	/// Handles a given [`TuiEvent`], returning an [`AppEvent`] if applicable.
	pub fn handle_tui_event(&mut self, event: TuiEvent) -> Option<AppEvent> {
		match event {
			TuiEvent::Hello => {
				info!("received init event! very considerate of you, kind tui");
				None
			},
			TuiEvent::Tick => {
				if self.input_buffer.is_empty() {
					return None;
				}
				let events = self.input_buffer.drain(..).collect();
				Some(AppEvent::UserInputs(events))
			},
			TuiEvent::Input(input) => {
				self.input_buffer.push(input);
				None
			},
			event => Some(event.try_into().unwrap()),
		}
	}
}
