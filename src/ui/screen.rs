//! A [Screen] to display content. Integral to Terminal Arcade's workings.

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
use enum_dispatch::enum_dispatch;
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

use crate::{
	core::terminal::BackendType,
	ui::{
		components::presets::{
			highlight_block,
			titled_ui_block,
			HIGHLIGHTED,
		},
		ConfigScreen,
		GameSearchScreen,
		MinesweeperSetupScreen,
		WelcomeScreen,
	},
};

/// A controls entry. The first element of the tuple is the key shortcut, while
/// the second element is the function (what it does in the context of the
/// screen).
pub type ControlsEntry = (&'static str, &'static str);

/// Returns a table containing information about key shortcuts.
#[must_use]
fn get_controls_table<'a>(extra_entries: Option<Vec<ControlsEntry>>) -> Table<'a> {
	let mut entries = extra_entries.unwrap_or_default();
	let mut default_shortcuts = vec![
		("Esc", "Closes this screen and returns to the previous one"),
		("Ctrl-Q", "Quits the application"),
	];
	entries.append(&mut default_shortcuts);
	Table::new(
		entries.into_iter().map(|entry| Row::new([Cell::new(entry.0), Cell::new(entry.1)])),
		&[
			Constraint::Ratio(1, 6), // shortcut
			Constraint::Ratio(5, 6), // function
		],
	)
	.block(highlight_block(titled_ui_block("Controls")))
	.highlight_spacing(HighlightSpacing::Always)
	.column_spacing(3)
	.header(
		Row::new(["Shortcut", "Function"]).style(HIGHLIGHTED.add_modifier(Modifier::UNDERLINED)),
	)
}

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
#[must_use]
pub struct ScreenState {
	/// Title of the screen, displayed on top by a surrounding block.
	pub title: &'static str,

	/// Kind of the screen.
	pub kind: ScreenKind,

	/// Open status of the screen - open or closed.
	pub open_status: OpenStatus,

	/// Open status of the *controls popup*.
	pub popup_open_status: OpenStatus,

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
			popup_open_status: OpenStatus::Closed,
			controls_entries,
			screen_created: None,
		}
	}
}

/// The trait for handling drawing on the terminal and receiving events from the
/// user. Use the associated ***screen state*** struct [`ScreenState`] to handle
/// screen/window stuff.
///
/// One should always start here when making a game/screen.
#[must_use]
#[enum_dispatch(Screens)]
pub trait Screen: Clone {
	/// Returns an initial screen state when this screen is first created.
	fn initial_state(&self) -> ScreenState;

	/// Called when an event is passed along to the screen,
	/// possibly from [`crate::TerminalArcade`], but also from whatever screen
	/// spawned this screen.
	fn event(&mut self, _event: &Event, _state: &mut ScreenState) -> anyhow::Result<()> {
		Ok(())
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
	fn render_screen(&mut self, frame: &mut Frame<'_>, state: &ScreenState);

	/// Renders the screen (not its children). The method also draws a
	/// screen-sized base block with a provided title by the trait.
	fn render(&mut self, frame: &mut Frame<'_>, state: &mut ScreenState) {
		let mut base_block = titled_ui_block(state.title);
		if state.screen_created.is_some() {
			base_block = base_block.style(Style::new().add_modifier(Modifier::DIM));
		}
		frame.render_widget(base_block, frame.size());
		self.render_screen(frame, state);
	}

	/// Draws the controls popup to the screen.
	/// This method is intended to be called whenever a shortcut is
	fn draw_controls_popup(&self, frame: &mut Frame<'_>, buffer: &mut Buffer, state: &ScreenState) {
		let frame_area = frame.size();
		let area = Rect {
			x: frame_area.width / 5,
			y: frame_area.height / 3,
			width: frame_area.width / 5 * 3,
			height: frame_area.height / 3,
		};
		Clear.render(area, buffer);
		frame.render_widget(get_controls_table(state.controls_entries.clone()), area);
	}
}

/// All screens implemented in Terminal Arcade.
#[enum_dispatch]
#[derive(Clone)]
#[allow(missing_docs)]
pub enum Screens {
	WelcomeScreen,
	ConfigScreen,
	GameSearchScreen,
	MinesweeperSetupScreen,
}
