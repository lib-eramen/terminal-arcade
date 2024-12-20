//! Utilities & primitives for the UI.

pub mod animation;

/// Wraps a given index to an array's length, Python-style.
pub fn wrap_index(index: isize, length: usize) -> usize {
	debug_assert_ne!(length, 0, "nothing to index when len == 0");
	if index < 0 {
		(length - index.unsigned_abs() % length) % length
	} else {
		index.unsigned_abs() % length
	}
}

#[cfg(test)]
mod test {
	use rstest::*;

	use super::*;

	#[rstest]
	fn wrap_index_does_nothing_for_indices_in_bound(
		#[values(1, 2, 5, 100, 1000, 0)] index: isize,
	) {
		assert_eq!(
			wrap_index(index, 1001),
			usize::try_from(index).unwrap(),
			"index is in bound, so should remain the same"
		);
	}

	#[rstest]
	#[should_panic(expected = "nothing to index when len == 0")]
	fn wrap_index_panics_for_length_0() {
		wrap_index(1, 0);
	}

	#[rstest]
	fn wrap_index_works_for_negative_indices(
		#[values(-1, -2, -5, -100)] index: isize,
	) {
		assert_eq!(
			wrap_index(index, 101),
			usize::try_from(101 + index).unwrap(),
			"negative index begins the other way around"
		);
	}

	#[rstest]
	#[case(10, 0)]
	#[case(11, 1)]
	#[case(15, 5)]
	#[case(20, 0)]
	#[case(33, 3)]
	#[case(101, 1)]
	#[case(-11, 9)]
	#[case(-15, 5)]
	#[case(-21, 9)]
	#[case(-33, 7)]
	#[case(-101, 9)]
	fn wrap_index_works_for_overflows(
		#[case] input: isize,
		#[case] output: usize,
	) {
		assert_eq!(wrap_index(input, 10), output, "wrapping index works!");
	}
}
