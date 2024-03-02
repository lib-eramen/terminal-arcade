//! A game-selection screen.
//! Users can scroll through the list with arrows to look for a game they want,
//! search a game by its name, or pick a game at random.
//!
//! oh jank warning btw

use std::cmp::{
	max,
	min,
};

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
use rand::Rng;
use ratatui::{
	layout::{
		Constraint,
		Direction,
		Layout,
		Rect,
	},
	Frame,
};

use crate::{
	core::terminal::BackendType,
	games::{
		all_games,
		games_by_keyword,
		get_game_by_name,
		Game,
		GameMetadata,
	},
	ui::{
		components::{
			game_select::{
				search_bottom_bar::render_search_bottom_bar,
				search_results::render_search_results,
				search_section::render_search_section,
			},
			scroll_tracker::ScrollTracker,
			ui_presets::{
				titled_ui_block,
				untitled_ui_block,
			},
		},
		Screen,
	},
};

/// Turns a character uppercase.
fn uppercase_char(c: char) -> char {
	c.to_uppercase().to_string().chars().next().unwrap()
}

/// Slices a, well, slice, getting the elements until there is no more to get,
/// or the slice range ends.
fn slice_until_end<T>(slice: &[T], start: usize, amount: usize) -> &[T] {
	if start >= slice.len() {
		return &[];
	}
	&slice[start..min(slice.len(), start + amount)]
}

/// The struct for the game selection screen.
pub struct GameSelectionScreen {
	/// The search term inputted by the user.
	search_term: Option<String>,

	/// The search results.
	search_results: Vec<GameMetadata>,

	/// Indicates whether a game has been chosen.
	selected_game: bool,

	/// The scroll tracker of this screen.
	scroll_tracker: ScrollTracker,

	/// Controls whether the controls help text are displayed.
	display_help_text: bool,

	/// The time spent to search and filter the results, in seconds.
	time_to_search_secs: f64,
}

impl Default for GameSelectionScreen {
	fn default() -> Self {
		let all_game_meta: Vec<GameMetadata> =
			all_games().into_iter().map(|game| game.metadata()).collect();
		let length = all_game_meta.len();

		Self {
			search_term: None,
			search_results: all_game_meta,
			selected_game: false,
			scroll_tracker: ScrollTracker::new(length as u64, Some(5)),
			display_help_text: true,
			time_to_search_secs: 0.0,
		}
	}
}

impl Screen for GameSelectionScreen {
	fn event(&mut self, event: &Event) -> anyhow::Result<()> {
		if let Event::Key(key) = event {
			match key.code {
				KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
					self.scroll_tracker.scroll_to_random();
				},
				KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
					self.clear_search_term();
				},
				KeyCode::Backspace => self.remove_one_character(),
				KeyCode::Char(character)
					if [KeyModifiers::SHIFT, KeyModifiers::NONE].contains(&key.modifiers) =>
				{
					self.add_character_to_term(character, key.modifiers);
				},
				KeyCode::Up => self.scroll_tracker.scroll_up(),
				KeyCode::Down => self.scroll_tracker.scroll_down(),
				KeyCode::Left => self.decrease_searches_shown(),
				KeyCode::Right => self.increase_searches_shown(),
				KeyCode::Tab => self.display_help_text = !self.display_help_text,
				KeyCode::Enter if self.scroll_tracker.is_selected() => self.selected_game = true,
				_ => {},
			}
		}
		Ok(())
	}

	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		let size = frame.size();
		frame.render_widget(titled_ui_block("Select a game!"), size);
		let chunks = self.game_selection_layout(size).split(size);

		let search_term = self.search_term.as_deref();
		render_search_section(frame, chunks[0], search_term, self.display_help_text);
		render_search_results(
			frame,
			chunks[1],
			search_term,
			slice_until_end(
				&self.search_results,
				self.scroll_tracker.start as usize,
				self.scroll_tracker.range.unwrap() as usize,
			),
			&self.scroll_tracker,
		);
		render_search_bottom_bar(
			frame,
			chunks[2],
			self.search_results_length(),
			self.time_to_search_secs,
			max(self.scroll_tracker.range.unwrap(), 5),
		);
	}

	fn screen_created(&mut self) -> Option<Box<dyn Screen>> {
		if !self.selected_game {
			return None;
		}
		let selected_index = self.scroll_tracker.selected?;
		let selection = &self.search_results[selected_index as usize];
		Some(get_game_by_name(selection.name())?.screen_created())
	}

	fn is_closing(&self) -> bool {
		self.selected_game
	}
}

impl GameSelectionScreen {
	/// Returns the layout for the game selection screen.
	#[must_use]
	fn game_selection_layout(&self, size: Rect) -> Layout {
		let search_section_height = if self.display_help_text { 8 } else { 3 };
		let used_ui_height = search_section_height + 3 + 2;
		let search_results_height =
			if used_ui_height >= size.height { 10 } else { size.height - used_ui_height };

		let constraints = vec![
			Constraint::Max(search_section_height), // Search bar/section
			Constraint::Max(search_results_height), // Search results
			Constraint::Max(3),                     // Search bottom info row
			Constraint::Max(0),                     /* Prevents elements from taking all
			                                         * remaining space. */
		];
		Layout::default()
			.direction(Direction::Vertical)
			.vertical_margin(1)
			.horizontal_margin(1)
			.constraints(constraints)
	}

	/// Gets the length of the search results.
	fn search_results_length(&self) -> usize {
		self.search_results.len()
	}

	/// Updates the search results.
	fn update_search_results(&mut self) {
		let timer = std::time::Instant::now();
		self.search_results = if let Some(ref term) = self.search_term {
			games_by_keyword(term.to_lowercase().as_str())
		} else {
			all_games()
		}
		.into_iter()
		.map(|game| game.metadata())
		.collect();
		self.scroll_tracker.set_length(self.search_results_length() as u64);
		self.time_to_search_secs = timer.elapsed().as_secs_f64();
	}

	/// Adds the character to the search term object, capping out at 256
	/// characters.
	fn add_character_to_term(&mut self, character: char, modifier: KeyModifiers) {
		let character =
			if modifier == KeyModifiers::SHIFT { uppercase_char(character) } else { character };
		match self.search_term {
			None => self.search_term = Some(character.to_string()),
			Some(ref mut term) if term.len() < 100 => term.push(character),
			Some(_) => panic!("Logic went flying all around the plane of existence"),
		}
		self.update_search_results();
	}

	/// Clears the search term.
	fn clear_search_term(&mut self) {
		self.search_term = None;
		self.update_search_results();
	}

	/// Pops one character from the search term (the top one), or does
	/// nothing if the term is empty.
	fn remove_one_character(&mut self) {
		if let Some(ref mut term) = self.search_term {
			term.pop();
			if term.is_empty() {
				self.search_term = None;
			}
		}
		self.update_search_results();
	}

	/// Increases the number of shown searches, capping out at 10.
	fn increase_searches_shown(&mut self) {
		let count = self.scroll_tracker.range.unwrap();
		if count < 10 {
			self.scroll_tracker.set_range(count + 1);
		}
	}

	/// Decreases the number of shown searches, capping out at 5.
	fn decrease_searches_shown(&mut self) {
		let count = self.scroll_tracker.range.unwrap();
		if count > 5 {
			self.scroll_tracker.set_range(count - 1);
		}
	}
}
