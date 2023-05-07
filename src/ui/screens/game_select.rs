//! A game-selection screen.
//! Users can scroll through the list with arrows to look for a game they want,
//! search a game by its name, or pick a game at random.

use std::cmp::min;

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
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
		GameMetadata,
	},
	ui::components::{
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
fn slice_until_end<T>(slice: &[T], start: usize, end: usize) -> &[T] {
	if start >= slice.len() {
		return &[];
	}
	&slice[start..min(slice.len(), end)]
}

/// The struct for the game selection screen.
#[derive(Default)]
pub struct GameSelectionScreen {
	/// The search term inputted by the user.
	search_term: Option<String>,

	/// The search results.
	search_results: Vec<GameMetadata>,

	/// The current choice index highlighted in the UI.
	selected_index: Option<usize>,

	/// The current index to start displaying the search results from.
	/// Is used for scrolling.
	start_index: usize,
}

impl Screen for GameSelectionScreen {
	fn event(&mut self, event: &Event) -> Outcome<()> {
		if let Event::Key(key) = event {
			match key.code {
				KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
					self.set_random_game();
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
					self.scroll_up();
				},
				KeyCode::Down => {
					self.scroll_down();
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

		let search_term = self.search_term.as_ref().map(|term| term.as_str());
		render_search_section(frame, chunks[0], search_term);

		let search_results: Vec<GameMetadata> = if let Some(term) = search_term {
			games_by_keyword(term.to_lowercase().as_str())
		} else {
			all_games()
		}
		.into_iter()
		.map(|game| game.metadata())
		.collect();
		render_search_results(
			frame,
			chunks[1],
			search_term,
			slice_until_end(&search_results, self.start_index, self.start_index + 5),
		);
	}
}

impl GameSelectionScreen {
	/// Returns the layout for the game selection screen.
	#[must_use]
	pub fn game_selection_layout() -> Layout {
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

	/// Updates the search results.
	fn update_search_results(&mut self) {
		self.search_results = if let Some(ref term) = self.search_term {
			games_by_keyword(term.to_lowercase().as_str())
		} else {
			all_games()
		}
		.into_iter()
		.map(|game| game.metadata())
		.collect()
	}

	/// Adds the character to the search term object, capping out at 256
	/// characters.
	fn add_character_to_term(&mut self, character: char, modifier: KeyModifiers) {
		let character =
			if modifier == KeyModifiers::SHIFT { uppercase_char(character) } else { character };
		match self.search_term {
			None => self.search_term = Some(character.to_string()),
			Some(ref mut term) if term.len() < 100 => term.push(character),
			Some(_) => panic!("Something went horribly wrong, woop woop"),
		}
		self.update_search_results();
	}

	/// Sets the entire UI to display a randomly generated game.
	fn set_random_game(&mut self) {
		todo!()
	}

	/// Clears the search term.
	fn clear_search_term(&mut self) {
		self.search_term = None;
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
	}

	/// Scrolls the list up.
	fn scroll_up(&mut self) {}

	/// Scrolls the list down.
	fn scroll_down(&mut self) {}
}
