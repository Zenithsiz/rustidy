//! Utilities

// Exports
pub use util::*;

// Imports
use {
	ast_util::delimited::Delimited,
	format::{WhitespaceConfig, WhitespaceFormat},
};

/// A value delimited by parenthesis
pub type Parenthesized<T> = Delimited<T, ast_token::ParenOpen, ast_token::ParenClose>;

/// A value delimited by brackets
pub type Bracketed<T> = Delimited<T, ast_token::BracketOpen, ast_token::BracketClose>;

/// A value delimited by braces
pub type Braced<T> = Delimited<T, ast_token::BracesOpen, ast_token::BracesClose>;

/// Single or indent formatting
#[derive(Clone, Copy, Debug)]
pub enum FmtSingleOrIndent {
	Single,
	Indent,
}

impl FmtSingleOrIndent {
	#[must_use]
	pub const fn prefix_ws(self) -> WhitespaceConfig {
		match self {
			Self::Single => Whitespace::SINGLE,
			Self::Indent => Whitespace::INDENT,
		}
	}
}

/// Remove or indent formatting
#[derive(Clone, Copy, Debug)]
pub enum FmtRemoveOrIndent {
	Remove,
	Indent,
}

impl FmtRemoveOrIndent {
	#[must_use]
	pub const fn prefix_ws(self) -> WhitespaceConfig {
		match self {
			Self::Remove => Whitespace::REMOVE,
			Self::Indent => Whitespace::INDENT,
		}
	}
}
