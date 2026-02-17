//! Whitespace

// Imports
use crate::{Arena, ArenaData, ArenaIdx, AstStr};

/// Whitespace
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Whitespace(pub ArenaIdx<WhitespaceInner>);

impl Whitespace {
	/// Creates an empty whitespace
	#[must_use]
	pub fn empty() -> Self {
		let inner = WhitespaceInner {
			first: PureWhitespace(AstStr::new("")),
			rest:  vec![],
		};
		let idx = ArenaIdx::new(inner);

		Self(idx)
	}

	/// Clears this whitespace
	pub fn clear(&mut self, input: &str) {
		let mut inner = self.0.get_mut();
		inner.first.0.replace(input, "");
		inner.rest.clear();
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WhitespaceInner {
	pub first: PureWhitespace,
	pub rest:  Vec<(Comment, PureWhitespace)>,
}

impl ArenaData for WhitespaceInner {
	const ARENA: &'static Arena<Self> = &ARENA;
}
static ARENA: Arena<WhitespaceInner> = Arena::new();

/// Comment
#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Comment {
	Line(LineComment),
	Block(BlockComment),
}

/// Block Comment
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BlockComment(pub AstStr);

/// Line comment
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineComment(pub AstStr);

/// Pure whitespace
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PureWhitespace(pub AstStr);
