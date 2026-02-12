//! Visibility

// Imports
use {
	super::{path::SimplePath, token, util::Parenthesized},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Visibility`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Visibility {
	pub pub_: token::Pub,
	#[format(prefix_ws = Whitespace::remove)]
	#[format(and_with = rustidy_format::format_option_with(Parenthesized::format_remove))]
	pub path: Option<Parenthesized<VisibilityPath>>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum VisibilityPath {
	Crate(token::Crate),
	Self_(token::SelfLower),
	Super(token::Super),
	In(VisibilityPathIn),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct VisibilityPathIn {
	pub in_:  token::In,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub path: SimplePath,
}
