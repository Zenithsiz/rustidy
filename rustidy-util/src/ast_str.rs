//! Ast string

// Imports
use {
	crate::{Arena, ArenaData, ArenaIdx, ArenaRef, AstRange, Config, Replacement},
	std::borrow::Cow,
};

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
	pub fn repr(&self) -> ArenaRef<'_, Self> {
		ARENA.get(&self.0)
	}

	/// Returns the string of this string
	#[must_use]
	pub fn str<'input>(&self, input: &'input str, config: &Config) -> Cow<'input, str> {
		match *self.repr() {
			AstStrRepr::AstRange(range) => range.str(input).into(),
			AstStrRepr::String(s) => s.into(),
			AstStrRepr::Replacement(ref replacement) => {
				let mut output = String::new();
				replacement.write(config, &mut output);
				output.into()
			},
		}
	}
}

impl ArenaData for AstStr {
	type Data = AstStrRepr;

	const ARENA: &'static Arena<Self> = &ARENA;
}

static ARENA: Arena<AstStr> = Arena::new();

#[derive(Debug, Clone)]
#[derive(derive_more::From)]
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum AstStrRepr {
	AstRange(AstRange),
	String(&'static str),
	Replacement(Replacement),
}

impl<'de> serde::Deserialize<'de> for AstStrRepr {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		// TODO: Should we try to deserialize a replacement?
		//       This impl is only used for testing the parsing output,
		//       which only contains ranges, so it might be unnecessary.
		AstRange::deserialize(deserializer).map(Self::AstRange)
	}
}
