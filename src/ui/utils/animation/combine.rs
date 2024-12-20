//! Utilities for combining frames together. Mostly useful for animating banners
//! of sorts. *Maybe only useful for animating banners of sorts.*

use std::{
	cmp::Ordering,
	collections::HashSet,
};

/// Method to resolve combining two distinct characters from each frames.
#[derive(Debug, Clone, Copy)]
pub enum MismatchCombineMethod {
	/// Enforce that there is a discard character between the two. It is
	/// an error or [`panic!`] other wise.
	EnforceDiscard,

	/// Use the earlier frame's character. If one or both of them are
	/// discardable, use the less non-discardable one.
	UseEarlier,

	/// Use the later frame's character. If one or both of them are
	/// discardable, use the less non-discardable one.
	UseLater,

	/// Assign the provided character in the place where there are two
	/// different characters.
	AssignChar(char),
}

/// A [`MismatchCombineMethod`] and a character that counts as a discardable
/// placeholder.
#[derive(Debug, Clone)]
pub struct FrameCombinator {
	/// Method to combine mismatched characters.
	method: MismatchCombineMethod,

	/// Placeholder characters that can be laid over—discardable.
	///
	/// Note that the order of the characters in this [`Vec`] is significant.
	///
	/// For example, given discard characters `[a, b, c]` *in that order*,
	/// combining `a` and `b` will result in `b`, even though both are discard
	/// characters. Similarly, `a` combined with `c` is `c`, because `a` is
	/// higher on the discard hierarchy than `c`. Any other character will take
	/// precedence over these discard characters, i.e. combining `d` with these
	/// characters will all result in `d`.
	///
	/// Combining discard levels can express that both characters have the same
	/// position in the discard hierarchy. This can be helpful in enforcing
	/// discards and assist in changing the frames so that combination behavior
	/// works.
	discard_chars: Vec<Vec<char>>,
}

impl FrameCombinator {
	/// Constructs a new frame combinator.
	pub fn new(
		method: MismatchCombineMethod,
		discard_chars: Vec<Vec<char>>,
	) -> Self {
		Self::validate_discard_chars(&discard_chars);
		Self {
			method,
			discard_chars,
		}
	}

	/// Check that no duplicates exist in [`Self::discard_chars`].
	fn validate_discard_chars(discard_chars: &[Vec<char>]) {
		discard_chars
			.iter()
			.flatten()
			.fold(HashSet::new(), |mut set, c| {
				assert!(
					set.insert(c),
					"cannot have duplicates of discard chars; found {c} twice"
				);
				set
			});
	}

	/// Combine two characters in the same position from two frames.
	pub fn combine(&self, earlier: char, later: char) -> char {
		if earlier == later {
			earlier
		} else if let Some(c) = self.try_discard(earlier, later) {
			c
		} else {
			self.apply_discard_method(earlier, later)
		}
	}

	/// Gets the position of a character in the discard hierarchy.
	fn get_discard_position(&self, c: char) -> Option<usize> {
		self.discard_chars
			.iter()
			.enumerate()
			.find(|(_, chars)| chars.contains(&c))
			.map(|(level, _)| level)
	}

	/// Tries to discard two given characters. If they are differently
	/// discardable, the non-discarded character is returned. This behavior
	/// is common across all discard methods.
	///
	/// This method assumes that `earlier != later`.
	fn try_discard(&self, earlier: char, later: char) -> Option<char> {
		debug_assert_ne!(earlier, later);
		Some(
			match (
				self.get_discard_position(earlier),
				self.get_discard_position(later),
			) {
				(Some(_), None) => later,
				(None, Some(_)) => earlier,
				(Some(e), Some(l)) => match e.cmp(&l) {
					Ordering::Less => later,
					Ordering::Greater => earlier,
					Ordering::Equal => return None,
				},
				(None, None) => return None,
			},
		)
	}

	/// Applies the discard method.
	///
	/// This method assumes that `earlier != later`.
	fn apply_discard_method(&self, earlier: char, later: char) -> char {
		debug_assert_ne!(earlier, later);
		match (self.method, earlier, later) {
			(MismatchCombineMethod::UseEarlier, e, _) => e,
			(MismatchCombineMethod::UseLater, _, l) => l,
			(MismatchCombineMethod::AssignChar(c), ..) => c,
			(MismatchCombineMethod::EnforceDiscard, e, l) => {
				self.enforce_mismatch_discard(e, l)
			},
		}
	}

	/// Resolves a mismatch between two potentially discardable
	/// characters—[`Self::EnforceDiscard`] behavior.
	///
	/// This method assumes that `earlier != later`.
	fn enforce_mismatch_discard(&self, earlier: char, later: char) -> char {
		debug_assert_ne!(earlier, later);
		let panic_message_data = format!("earlier: {earlier}, later: {later}");
		match (
			self.get_discard_position(earlier),
			self.get_discard_position(later),
		) {
			(Some(_), None) => later,
			(None, Some(_)) => earlier,
			(Some(e), Some(l)) => match e.cmp(&l) {
				Ordering::Less => later,
				Ordering::Greater => earlier,
				Ordering::Equal => panic!(
					"cannot discard either character, as both are equally \
					 discardable; {panic_message_data}"
				),
			},
			(None, None) => panic!(
				"cannot discard either character, as both are \
				 non-discardable; {panic_message_data}"
			),
		}
	}
}

#[cfg(test)]
mod test {
	use rstest::*;

	use super::*;

	#[rstest]
	fn mismatch_combine_method_common_cases(
		#[values(
			MismatchCombineMethod::EnforceDiscard,
			MismatchCombineMethod::UseEarlier,
			MismatchCombineMethod::UseLater,
			MismatchCombineMethod::AssignChar('%')
		)]
		method: MismatchCombineMethod,
	) {
		let combinator = FrameCombinator::new(method, vec![vec![' ']]);
		let method_prefix = format!("`{method:?}` ");
		assert_eq!(
			combinator.combine('a', ' '),
			'a',
			"{method_prefix} discarding space for 'a'"
		);
		assert_eq!(
			combinator.combine(' ', 'b'),
			'b',
			"{method_prefix} discarding space for 'b'"
		);
		assert_eq!(
			combinator.combine('x', 'x'),
			'x',
			"{method_prefix} returning the char when both are the same"
		);
		assert_eq!(
			combinator.combine(' ', ' '),
			' ',
			"{method_prefix} returning the same discard char"
		);
	}

	#[rstest]
	#[should_panic(expected = "cannot discard either character, as both are \
	                           non-discardable; earlier: a, later: b")]
	fn enforce_discard_panics_without_discard_chars() {
		FrameCombinator::new(MismatchCombineMethod::EnforceDiscard, vec![
			vec![' '],
		])
		.combine('a', 'b');
	}

	#[rstest]
	#[should_panic(expected = "cannot discard either character, as both are \
	                           equally discardable; earlier: !, later: @")]
	fn discarding_equally_discardable_chars_panics() {
		FrameCombinator::new(MismatchCombineMethod::EnforceDiscard, vec![
			vec![' '],
			vec!['!', '@'],
		])
		.combine('!', '@');
	}

	#[rstest]
	fn discard_enforce_with_multiple_discard_levels_works() {
		let combinator =
			FrameCombinator::new(MismatchCombineMethod::EnforceDiscard, vec![
				vec![' '],
				vec!['!', '@'],
			]);
		for c in [' ', '!', '@'] {
			assert_eq!(
				combinator.combine('a', c),
				'a',
				"combining 'a' with one of the discard chars should return 'a'"
			);
		}
		assert_eq!(
			combinator.combine(' ', '!'),
			'!',
			"discarding space for ! as space is more discardable"
		);
		assert_eq!(
			combinator.combine('@', ' '),
			'@',
			"discarding space for @ as space is more discardable"
		);
	}

	#[rstest]
	fn use_earlier_or_later_works() {
		assert_eq!(
			FrameCombinator::new(MismatchCombineMethod::UseEarlier, vec![
				vec![' ']
			])
			.combine('a', 'b'),
			'a',
			"using earlier character 'a'",
		);
		assert_eq!(
			FrameCombinator::new(MismatchCombineMethod::UseLater, vec![vec![
				' '
			]])
			.combine('a', 'b'),
			'b',
			"using later character 'b'"
		);
	}

	#[rstest]
	fn assign_char_works(#[values('x', 'a', 'b')] c: char) {
		assert_eq!(
			FrameCombinator::new(MismatchCombineMethod::AssignChar(c), vec![
				vec![' ']
			])
			.combine('a', 'b'),
			c,
			"should assign {c} when both aren't discardable"
		);
	}
}
