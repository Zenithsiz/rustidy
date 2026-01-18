//! Path

// Imports
use {
	super::{
		ident::Identifier,
		punct::{self, Punctuated},
		token,
	},
	crate::{Format, Print},
	core::fmt::Debug,
	rustidy_parse::Parse,
};

/// `SimplePath`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a simple path")]
pub struct SimplePath {
	pub prefix:   Option<token::PathSep>,
	#[format(and_with(expr = Format::prefix_ws_remove, if = self.prefix.is_some()))]
	#[format(and_with = punct::format(Format::prefix_ws_remove, Format::prefix_ws_remove))]
	pub segments: Punctuated<SimplePathSegment, token::PathSep>,
}

/// `SimplePathSegment`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum SimplePathSegment {
	Super(token::Super),
	SelfLower(token::SelfLower),
	Crate(token::Crate),
	DollarCrate(token::DollarCrate),
	Ident(Identifier),
}
