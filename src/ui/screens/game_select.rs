//! A game-selection screen.
//! Users can scroll through the list with arrows to look for a game they want,
//! search a game by its name, or pick a game at random.
//!
//! JANKY CODE WARNING! The functions that control the scrolling of the game
//! list is quite janky, to say the least. Beware of burning your eyes! Use a
//! pair of protective lab goggles to protect yourself from the eye-scorchingly
//! horrible code quality! I as the author of the code is not responsible for
//! any kinds of impact to physical health caused by looking at said janky code!

#![allow(clippy::cast_possible_truncation)]
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
	},
	Frame,
};

use super::{
	check_escape_key,
	Screen,
};
use crate::{
	core::{
		terminal::BackendType,
		Outcome,
	},
	game::{
		all_games,
		games_by_keyword,
		get_game_by_name,
		Game,
		GameMetadata,
	},
	ui::components::{
		scroll_tracker::ScrollTracker,
		search_results::render_search_results,
		search_section::render_search_section,
		ui_presets::{
			titled_ui_block,
			untitled_ui_block,
		},
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
		}
	}
}

impl Screen for GameSelectionScreen {
	fn event(&mut self, event: &Event) -> Outcome<()> {
		if let Event::Key(key) = event {
			match key.code {
				KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
					self.scroll_tracker.scroll_to_random();
				},
				KeyCode::Backspace => {
					self.remove_one_character();
				},
				KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
					self.clear_search_term();
				},
				KeyCode::Char(character)
					if [KeyModifiers::SHIFT, KeyModifiers::NONE].contains(&key.modifiers) =>
				{
					self.add_character_to_term(character, key.modifiers);
				},
				KeyCode::Up => {
					self.scroll_tracker.scroll_up();
				},
				KeyCode::Down => {
					self.scroll_tracker.scroll_down();
				},
				KeyCode::Enter => {
					if self.scroll_tracker.is_selected() {
						self.selected_game = true;
					}
				},
				_ => {},
			}
		}
		Ok(())
	}

	fn draw_ui(&self, frame: &mut Frame<'_, BackendType>) {
		let size = frame.size();
		frame.render_widget(titled_ui_block("Select a game!"), size);
		let chunks = Self::game_selection_layout().split(size);

		let search_term = self.search_term.as_deref();
		render_search_section(frame, chunks[0], search_term);

		render_search_results(
			frame,
			chunks[1],
			search_term,
			slice_until_end(&self.search_results, self.scroll_tracker.start as usize, 5),
			&self.scroll_tracker,
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
	fn game_selection_layout() -> Layout {
		Layout::default()
			.direction(Direction::Vertical)
			.vertical_margin(1)
			.horizontal_margin(1)
			.constraints(
				[
					Constraint::Ratio(1, 7), // For search section (search bar and controls)
					Constraint::Ratio(6, 7), // For search results
				]
				.as_ref(),
			)
	}

	/// Gets the length of the search results.
	fn search_results_length(&self) -> usize {
		self.search_results.len()
	}

	/// Updates the search results.
	fn update_search_results(&mut self) {
		self.search_results = if let Some(ref term) = self.search_term {
			games_by_keyword(term.to_lowercase().as_str())
		} else {
			all_games()
		}
		.into_iter()
		.map(|game| game.metadata())
		.collect();
		self.scroll_tracker.set_length(self.search_results_length() as u64);
	}

	/// Adds the character to the search term object, capping out at 256
	/// characters.
	fn add_character_to_term(&mut self, character: char, modifier: KeyModifiers) {
		let character =
			if modifier == KeyModifiers::SHIFT { uppercase_char(character) } else { character };
		match self.search_term {
			None => self.search_term = Some(character.to_string()),
			Some(ref mut term) if term.len() < 100 => term.push(character),
			Some(_) => panic!("Logic went flying all around the place"),
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
}
