//! Parser range

// Imports
use crate::ParserPos;

/// Parser range
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParserRange {
	pub start: ParserPos,
	pub end:   ParserPos,
}

impl ParserRange {
	/// Creates a parser range from a start and end position
	#[must_use]
	pub const fn new(start: ParserPos, end: ParserPos) -> Self {
		Self { start, end }
	}

	/// Returns the length of this range
	#[must_use]
	pub const fn len(&self) -> usize {
		self.end.0 - self.start.0
	}

	/// Returns if this range is empty
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Slices the input string with this range
	#[must_use]
	pub fn str<'input>(&self, input: &'input str) -> &'input str {
		&input[self.start.0..self.end.0]
	}

	/// Slices the input string before this range
	#[must_use]
	pub fn str_before(self, s: &str) -> &str {
		&s[..self.start.0]
	}
}
