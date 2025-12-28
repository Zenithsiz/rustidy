//! Path

// Imports
use {
	super::{ident::Identifier, punct::Punctuated, token},
	crate::{Format, Print, parser::Parse},
	core::fmt::Debug,
};

/// `SimplePath`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a simple path")]
pub struct SimplePath {
	pub prefix:   Option<token::PathSep>,
	pub segments: Punctuated<SimplePathSegment, token::PathSep>,
}

/// `SimplePathSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum SimplePathSegment {
	Super(token::Super),
	SelfLower(token::SelfLower),
	Crate(token::Crate),
	DollarCrate(token::DollarCrate),
	Ident(Identifier),
}
