//! Whitespace

// Imports
use crate::{ArenaData, ArenaIdx, AstStr};

/// Whitespace
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Whitespace(pub ArenaIdx<WhitespaceInner>);

impl Whitespace {
	/// Creates an empty whitespace
	#[must_use]
	pub fn empty() -> Self {
		let inner = WhitespaceInner {
			first: PureWhitespace(AstStr::new("")),
			rest: vec![],
		};
		let idx = ArenaIdx::new(inner);

		Self(idx)
	}

	/// Clears this whitespace
	pub fn clear(&mut self, input: &str) {
		self.0.first.0.replace(input, "");
		self.0.rest.clear();
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WhitespaceInner {
	pub first: PureWhitespace,
	pub rest:  Vec<(Comment, PureWhitespace)>,
}

/// Comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Comment {
	Line(LineComment),
	Block(BlockComment),
}

/// Block Comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BlockComment(pub AstStr);

/// Line comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LineComment(pub AstStr);

/// Pure whitespace
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PureWhitespace(pub AstStr);
