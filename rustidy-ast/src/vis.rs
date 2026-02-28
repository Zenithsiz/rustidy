//! Visibility

// Imports
use {
	super::{path::SimplePath, util::Parenthesized},
	ast_literal::token,
	ast_util::delimited,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `Visibility`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Visibility {
	pub pub_: token::Pub,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtRemove)]
	pub path: Option<Parenthesized<VisibilityPath>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum VisibilityPath {
	Crate(token::Crate),
	Self_(token::SelfLower),
	Super(token::Super),
	In(VisibilityPathIn),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct VisibilityPathIn {
	pub in_:  token::In,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub path: SimplePath,
}
