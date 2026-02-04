//! Ast utils

// Features
#![feature(never_type, coverage_attribute, yeet_expr, anonymous_lifetime_in_impl_trait)]

// Modules
pub mod at_least;
pub mod delimited;
pub mod follows;
pub mod ident;
pub mod line;
pub mod longest;
pub mod not_follows;
pub mod punct;
pub mod whitespace;

// Exports
pub use self::{
	at_least::AtLeast1,
	delimited::Delimited,
	follows::Follows,
	ident::{Identifier, IdentifierOrKeyword, NonKeywordIdentifier, RawIdentifier},
	line::{RemainingBlockComment, RemainingLine},
	longest::Longest,
	not_follows::NotFollows,
	punct::{Punctuated, PunctuatedTrailing},
	whitespace::Whitespace,
};
