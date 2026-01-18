//! Visibility

// Imports
use {
	super::{delimited::Parenthesized, path::SimplePath, token},
	crate::{Format, Print, format},
	rustidy_parse::Parse,
};

/// `Visibility`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Visibility {
	pub pub_: token::Pub,
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = format::format_option_with(Parenthesized::format_remove))]
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub path: SimplePath,
}
