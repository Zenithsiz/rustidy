//! Delimited

// Imports
use {
	super::token,
	crate::{Format, Parse, Print},
};

/// A value `T` delimited by prefix `L` and suffix `R`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Delimited<T, L, R> {
	pub prefix: L,
	// TODO: Should we always remove all tags here?
	#[parse(without_tags)]
	pub value:  T,
	pub suffix: R,
}

/// A value delimited by parenthesis
pub type Parenthesized<T> = Delimited<T, token::ParenOpen, token::ParenClose>;

/// A value delimited by brackets
pub type Bracketed<T> = Delimited<T, token::BracketOpen, token::BracketClose>;

/// A value delimited by braces
pub type Braced<T> = Delimited<T, token::BracesOpen, token::BracesClose>;
