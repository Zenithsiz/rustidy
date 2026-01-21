//! Ast string

// Imports
use crate::{Arena, ArenaData, ArenaIdx, AstRange};

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
			AstStrRepr::String(s) => s,
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
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum AstStrRepr {
	AstRange(AstRange),
	String(&'static str),
}

impl<'de> serde::Deserialize<'de> for AstStrRepr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		AstRange::deserialize(deserializer).map(Self::AstRange)
	}
}
