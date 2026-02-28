//! Utilities

// Imports
use {
	rustidy_ast_literal::token,
	rustidy_ast_util::delimited::Delimited,
	rustidy_format::{WhitespaceConfig, WhitespaceFormat},
	rustidy_util::Whitespace,
};

/// A value delimited by parenthesis
pub type Parenthesized<T> = Delimited<T, token::ParenOpen, token::ParenClose>;

/// A value delimited by brackets
pub type Bracketed<T> = Delimited<T, token::BracketOpen, token::BracketClose>;

/// A value delimited by braces
pub type Braced<T> = Delimited<T, token::BracesOpen, token::BracesClose>;

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
