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
	/// Creates a new ast string
	pub fn new(repr: impl Into<AstStrRepr>) -> Self {
		Self(ARENA.push(repr.into()))
	}

	/// Creates a new 0-sized ast string from a position
	pub fn empty_at(pos: AstPos) -> Self {
		Self::new(AstRange { start: pos, end: pos })
	}

	/// Returns the inner representation of this string
	#[must_use]
	pub fn repr(&self) -> AstStrRepr {
		*ARENA.get(&self.0)
	}

	/// Returns the string of this string
	#[must_use]
	pub fn str<'input>(&self, input: &'input str) -> &'input str {
		match self.repr() {
			AstStrRepr::AstRange(range) => range.str(input),
		}
	}
}

impl ArenaData for AstStr {
	type Data = AstStrRepr;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<AstStr> = Arena::new();

#[derive(Debug, Clone, Copy)]
#[derive(derive_more::From)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum AstStrRepr {
	AstRange(AstRange),
}
