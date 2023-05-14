//! Components for a setup screen.

use std::ops::{
	Add,
	Div,
	Mul,
	Sub,
};

use crossterm::event::KeyCode;
use derive_builder::Builder;
use derive_new::new;
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
		Rect,
	},
	style::{
		Color,
		Style,
	},
	widgets::{
		BorderType,
		Paragraph,
	},
	Frame,
};

use crate::{
	core::terminal::BackendType,
	ui::{
		components::{
			scroll_tracker::ScrollTracker,
			ui_presets::untitled_ui_block,
		},
		util::stylize,
	},
};

/// A [setup question](SetupQuestion) that has a range-based answer.
#[derive(new, Builder)]
pub struct RangeQuestion {
	/// The label/description of the question.
	label: String,

	/// The maximum value the answer can be.
	max: i64,

	/// The minimum value the answer can be.
	min: i64,

	/// The step in which the answer moves in.
	step: i64,

	/// The current answer.
	answer: i64,
}

impl RangeQuestion {
	/// Decreases the answer by the range's step if it does not go below the
	/// minimum.
	pub fn decrease(&mut self) {
		let new_answer = self.answer - self.step;
		if new_answer >= self.min {
			self.answer = new_answer;
		}
	}

	/// Increases the answer by the range's step if it does not exceed the
	/// maximum.
	pub fn increase(&mut self) {
		let new_answer = self.answer + self.step;
		if new_answer <= self.max {
			self.answer = new_answer;
		}
	}
}

/// A [setup question](SetupQuestion) that has a toggled, boolean answer.
#[derive(new)]
pub struct ToggleQuestion {
	/// The label/description of the question.
	label: String,

	/// The current state of the answer.
	answer: bool,
}

impl ToggleQuestion {
	/// Toggles the answer.
	pub fn toggle(&mut self) {
		self.answer = !self.answer;
	}
}

/// A [setup question](SetupQuestion) that has an option-based answer.
pub struct OptionsQuestion {
	/// The label/description of the question.
	label: String,

	/// The provided options to choose from.
	options: Vec<String>,

	/// The scroll tracker to keep track of where the answer is at.
	scroll_tracker: ScrollTracker,
}

impl OptionsQuestion {
	/// Constructs a new option-based setup question.
	#[must_use]
	pub fn new(label: String, options: Vec<String>) -> Self {
		let options_count = options.len();
		Self {
			label,
			options,
			scroll_tracker: ScrollTracker::new(options_count as u64, None),
		}
	}

	/// Returns whether this question has been answered.
	#[must_use]
	pub fn is_answered(&self) -> bool {
		self.get_answer().is_some()
	}

	/// Gets the answer of this question.
	#[must_use]
	pub fn get_answer(&self) -> Option<String> {
		self.scroll_tracker.selected.map(|index| self.options[index as usize].clone())
	}

	/// Moves the selection pointer to the left.
	pub fn select_left(&mut self) {
		self.scroll_tracker.scroll_up();
	}

	/// Moves the selection pointer to the right.
	pub fn select_right(&mut self) {
		self.scroll_tracker.scroll_down();
	}
}

/// A game setup question.
pub enum SetupQuestion {
	/// A range-based question.
	Range(RangeQuestion),

	/// A range-based question.
	Toggle(ToggleQuestion),

	/// A range-based question.
	Options(OptionsQuestion),
}

/// Renders a list of setup questions.
pub fn render_setup_questions(
	questions: &[SetupQuestion],
	frame: &mut Frame<'_, BackendType>,
	layout: &Layout,
	size: Rect,
) {
	for (question, sub_size) in questions.iter().zip(layout.split(size).iter()) {
		render_setup_question(question, frame, *sub_size);
	}
}

/// Renders a setup question.
pub fn render_setup_question(
	question: &SetupQuestion,
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
) {
	frame.render_widget(untitled_ui_block(), size);
	match question {
		SetupQuestion::Range(ref range_question) => {
			render_range_question(range_question, frame, size);
		},
		SetupQuestion::Toggle(ref toggle_question) => {
			render_toggle_question(toggle_question, frame, size);
		},
		SetupQuestion::Options(ref options_question) => {
			render_options_question(options_question, frame, size);
		},
	}
}

/// Renders a [range-based setup question](RangeSetupQuestion).
pub fn render_range_question(
	question: &RangeQuestion,
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
) {
	let mut constraints = vec![Constraint::Ratio(7, 10)];
	constraints.append(&mut vec![Constraint::Ratio(1, 10); 3]);
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.horizontal_margin(1)
		.vertical_margin(0)
		.constraints(constraints)
		.split(size);
	frame.render_widget(untitled_ui_block(), size);
	frame.render_widget(
		Paragraph::new(stylize(&question.label)).alignment(Alignment::Left),
		chunks[0],
	);
	frame.render_widget(
		Paragraph::new(question.min.to_string())
			.alignment(Alignment::Center)
			.block(untitled_ui_block()),
		chunks[1],
	);
	frame.render_widget(
		Paragraph::new(stylize(&question.answer.to_string()))
			.alignment(Alignment::Center)
			.block(untitled_ui_block().border_type(BorderType::Thick)),
		chunks[2],
	);
	frame.render_widget(
		Paragraph::new(question.max.to_string())
			.alignment(Alignment::Center)
			.block(untitled_ui_block()),
		chunks[3],
	);
}

/// Renders a [toggle-based setup question](ToggleSetupQuestion).
pub fn render_toggle_question(
	question: &ToggleQuestion,
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
) {
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.margin(1)
		.constraints([Constraint::Ratio(4, 5), Constraint::Ratio(1, 5)].as_ref())
		.split(size);
	let toggled_button = Paragraph::new(if question.answer { "True" } else { "False" })
		.alignment(Alignment::Center)
		.block(
			untitled_ui_block().style(Style::default().bg(if question.answer {
				Color::LightGreen
			} else {
				Color::LightRed
			})),
		)
		.style(Style::default().fg(Color::Black));
	frame.render_widget(Paragraph::new(stylize(&question.label)), chunks[0]);
	frame.render_widget(toggled_button, chunks[0]);
}

/// Renders an [options-based setup question](OptionsSetupQuestion).
pub fn render_options_question(
	question: &OptionsQuestion,
	frame: &mut Frame<'_, BackendType>,
	size: Rect,
) {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(1)
		.constraints([Constraint::Max(3); 2].as_ref())
		.split(size);
	frame.render_widget(Paragraph::new(stylize(&question.label)), chunks[0]);

	let option_count = question.options.len();
	let option_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.vertical_margin(1)
		.horizontal_margin(0)
		.constraints(vec![
			Constraint::Ratio(1, option_count as u32);
			option_count
		])
		.split(chunks[1]);
	for (index, option) in question.options.iter().enumerate() {
		let mut block = untitled_ui_block();
		if question.scroll_tracker.selected.map_or(false, |selected| selected == index as u64) {
			block = block
				.border_type(BorderType::Thick)
				.border_style(Style::default().fg(Color::LightRed));
		}
		frame.render_widget(
			Paragraph::new(stylize(option)).block(block),
			option_chunks[0],
		);
	}
}

/// Handles a key action that mutates the state of a question.
pub fn handle_key_action(question: &mut SetupQuestion, key_code: KeyCode) {
	match question {
		SetupQuestion::Range(ref mut range_question) => match key_code {
			KeyCode::Left => range_question.decrease(),
			KeyCode::Right => range_question.increase(),
			_ => (),
		},
		SetupQuestion::Toggle(ref mut toggle_question) => {
			if key_code == KeyCode::Enter {
				toggle_question.toggle();
			}
		},
		SetupQuestion::Options(ref mut options_question) => match key_code {
			KeyCode::Left => options_question.select_left(),
			KeyCode::Right => options_question.select_right(),
			_ => (),
		},
	}
}
