//! Ast string

// Imports
use crate::{Arena, ArenaData, ArenaIdx, AstPos, AstRange};

/// Ast string
// TODO: Add an "empty" position for newly created ast nodes?
#[derive(PartialEq, Eq, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[must_use = "Ast output must not be discarded"]
pub struct AstStr(pub ArenaIdx<Self>);

impl AstStr {
	/// Creates a new ast string from a range
	pub fn new(range: AstRange) -> Self {
		Self(ARENA.push(range))
	}

	/// Creates a new 0-sized ast string from a position
	pub fn empty_at(pos: AstPos) -> Self {
		Self::new(AstRange { start: pos, end: pos })
	}

	/// Returns the ast range of this string
	#[must_use]
	pub fn range(&self) -> AstRange {
		*ARENA.get(&self.0)
	}
}

impl ArenaData for AstStr {
	type Data = AstRange;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<AstStr> = Arena::new();
