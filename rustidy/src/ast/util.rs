//! Utilities

// Imports
use {super::token, rustidy_ast_util::delimited::Delimited};

/// A value delimited by parenthesis
pub type Parenthesized<T> = Delimited<T, token::ParenOpen, token::ParenClose>;

/// A value delimited by brackets
pub type Bracketed<T> = Delimited<T, token::BracketOpen, token::BracketClose>;

/// A value delimited by braces
pub type Braced<T> = Delimited<T, token::BracesOpen, token::BracesClose>;
