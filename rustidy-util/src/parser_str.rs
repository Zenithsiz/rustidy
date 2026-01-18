//! Parse string

// Imports
use crate::{Arena, ArenaData, ArenaIdx, ParserPos, ParserRange};

/// Parser string
#[derive(PartialEq, Eq, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[must_use = "Parser output must not be discarded"]
pub struct ParserStr(pub ArenaIdx<Self>);

impl ParserStr {
	/// Creates a new parser string from a range
	pub fn new(range: ParserRange) -> Self {
		Self(ARENA.push(range))
	}

	/// Creates a new 0-sized parser string from a position
	pub fn empty_at(pos: ParserPos) -> Self {
		Self::new(ParserRange { start: pos, end: pos })
	}

	/// Returns the parser range of this string
	#[must_use]
	pub fn range(&self) -> ParserRange {
		*ARENA.get(&self.0)
	}
}

impl ArenaData for ParserStr {
	type Data = ParserRange;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<ParserStr> = Arena::new();
