//! Path

// Imports
use {
	super::token,
	core::fmt::Debug,
	rustidy_ast_util::{Identifier, Punctuated, punct},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `SimplePath`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a simple path")]
pub struct SimplePath {
	pub prefix:   Option<token::PathSep>,
	#[format(before_with(expr = Format::prefix_ws_remove, if = self.prefix.is_some()))]
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
