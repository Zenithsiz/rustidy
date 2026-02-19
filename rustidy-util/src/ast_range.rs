//! Ast range

// Imports
use crate::AstPos;

/// Ast range
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AstRange {
	pub start: AstPos,
	pub end:   AstPos,
}

impl AstRange {
	/// Creates an ast range from a start and end position
	#[must_use]
	pub const fn new(start: AstPos, end: AstPos) -> Self {
		Self {
			start,
			end
		}
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
	pub fn str_before<'input>(&self, input: &'input str) -> &'input str {
		&input[..self.start.0]
	}
}
