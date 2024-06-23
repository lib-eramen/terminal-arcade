//! Module for screens used in Terminal Arcade. See [Screen] to get started.

pub mod config;
pub mod controls_popup;
pub mod game_select;
pub mod games;
pub mod welcome;

pub use config::ConfigScreen;
pub use controls_popup::ControlsPopup;
use crossterm::event::{
	Event,
	KeyCode,
	KeyEvent,
	KeyModifiers,
};
use enum_dispatch::enum_dispatch;
pub use game_select::GameSearchScreen;
pub use games::*;
use ratatui::{
	buffer::Buffer,
	layout::{
		Constraint,
		Rect,
	},
	style::{
		Modifier,
		Style,
	},
	text::Text,
	widgets::{
		Cell,
		Clear,
		HighlightSpacing,
		Row,
		Table,
		Widget,
	},
	Frame,
};
pub use welcome::WelcomeScreen;

use crate::ui::components::{
	presets::{
		highlight_block,
		titled_ui_block,
		HIGHLIGHTED,
	},
	screen_base_block::screen_base_block,
};

/// A controls entry. The first element of the tuple is the key shortcut, while
/// the second element is the function (what it does in the context of the
/// screen).
pub type ControlsEntry = (&'static str, &'static str);

/// Open status of the screen.
#[derive(Clone, Copy, PartialEq, Eq)]
#[must_use]
#[allow(missing_docs)]
pub enum OpenStatus {
	Open,
	Closed,
}

impl OpenStatus {
	/// Returns the toggled state.
	pub fn toggled(self) -> Self {
		match self {
			OpenStatus::Open => OpenStatus::Closed,
			OpenStatus::Closed => OpenStatus::Open,
		}
	}
}

/// Type of the screen, normal or popup.
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ScreenKind {
	Normal,
	Popup,
}

/// State of a screen. Preferably, this struct is handled and mutated by an
/// implementor of [Screen] itself, and not an overlying structure.
#[derive(Clone)]
#[must_use]
pub struct ScreenState {
	/// Title of the screen, displayed on top by a surrounding block.
	pub title: &'static str,

	/// Kind of the screen.
	pub kind: ScreenKind,

	/// Open status of the screen - open or closed.
	pub open_status: OpenStatus,

	/// Extra controls specific to this page, to be displayed in the controls
	/// popup.
	pub controls_entries: Option<Vec<ControlsEntry>>,

	/// Screen to be created and to be spawned.
	pub screen_created: Option<Screens>,
}

impl ScreenState {
	/// Creates a new screen state instance, with itself open and the popup
	/// closed.
	pub fn new(
		title: &'static str,
		kind: ScreenKind,
		controls_entries: Option<Vec<ControlsEntry>>,
	) -> Self {
		Self {
			title,
			kind,
			open_status: OpenStatus::Open,
			controls_entries,
			screen_created: None,
		}
	}

	/// Sets the [`Self::screen_created`] property, given a screen.
	pub fn set_screen_created(&mut self, screen: Screens) {
		self.screen_created = Some(screen);
	}
}

/// The trait for handling drawing on the terminal and receiving events from the
/// user. Use the associated ***screen state*** struct [`ScreenState`] to handle
/// screen/window stuff.
///
/// One should always start here when making a game/screen. To start, implement
/// [`Self::initial_state`], as well as [`Self::event_screen`] and
/// [`Self::render_ui`].
#[must_use]
#[enum_dispatch]
pub trait Screen {
	/// Returns an initial screen state when this screen is first created.
	fn initial_state(&self) -> ScreenState;

	/// Handles an input event.
	/// Using this method directly is discouraged - [`Self::event`] handles
	/// default shortcuts for every screen as well.
	fn handle_event(&mut self, event: &Event, state: &mut ScreenState) -> anyhow::Result<()>;

	/// Called when an input event is received.
	/// In addition to the events that [`Self::event_screen`] handles, this
	/// method also handles two extra events:
	/// - On \[Esc\], closes this screen.
	/// - On \[Ctrl\]+\[H\], displays the controls popup only when the screen is
	///   of [`ScreenKind::Normal`] kind.
	fn event(&mut self, event: &Event, state: &mut ScreenState) -> anyhow::Result<()> {
		if let Event::Key(ref key) = event {
			match key.code {
				KeyCode::Char('h')
					if key.modifiers == KeyModifiers::CONTROL
						&& state.kind == ScreenKind::Normal =>
				{
					state.set_screen_created(
						ControlsPopup::new(state.controls_entries.clone()).into(),
					);
				},
				KeyCode::Esc => {
					state.open_status = OpenStatus::Closed;
				},
				_ => {},
			}
		}
		self.handle_event(event, state)
	}

	/// Called when the screen is being closed.
	/// This can be called when the entire application is being quit (in the
	/// proper manner, of course, not through a crash or a panic).
	fn close(&mut self) -> anyhow::Result<()> {
		Ok(())
	}

	/// Renders ***this*** screen's UI.
	/// Using this method directly is discouraged - [`Self::render`] handles
	/// rendering its popups as well.
	fn render_ui(&self, frame: &mut Frame<'_>, state: &ScreenState);

	/// Renders the screen (not its children). The method also draws a
	/// screen-sized base block with a provided title by the trait.
	fn render(&mut self, frame: &mut Frame<'_>, state: &mut ScreenState, focused: bool) {
		if state.kind == ScreenKind::Normal {
			let mut base_block = screen_base_block(state.title);
			if !focused {
				base_block = base_block.style(Style::new().add_modifier(Modifier::DIM));
			}
			frame.render_widget(base_block, frame.size());
		}
		self.render_ui(frame, state);
	}
}

/// A wrapper struct for a screen and its state.
#[derive(Clone)]
#[must_use]
pub struct ScreenAndState {
	/// The screen itself.
	pub screen: Screens,

	/// State associated with the screen.
	pub state: ScreenState,
}

impl ScreenAndState {
	/// Creates a new screen and state object.
	pub fn new(screen: Screens) -> Self {
		let state = screen.initial_state();
		Self { screen, state }
	}

	/// Closes the screen.
	pub fn close(&mut self) -> anyhow::Result<()> {
		self.state.open_status = OpenStatus::Closed;
		self.screen.close()
	}
}

/// All screens implemented in Terminal Arcade.
#[enum_dispatch(Screen)]
#[derive(Clone)]
#[allow(missing_docs)]
pub enum Screens {
	ControlsPopup(ControlsPopup),
	WelcomeScreen(WelcomeScreen),
	ConfigScreen(ConfigScreen),
	GameSearchScreen(GameSearchScreen),
	MinesweeperSetupScreen(MinesweeperSetupScreen),
}

impl From<Screens> for ScreenAndState {
	fn from(screen: Screens) -> Self {
		ScreenAndState::new(screen)
	}
}
