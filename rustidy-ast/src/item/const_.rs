//! Constants

// Imports
use {
	crate::{expr::Expression, token, ty::Type},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ConstantItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstantItem {
	pub const_: token::Const,
	#[format(prefix_ws = Whitespace::set_single)]
	pub name:   ConstantItemName,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::remove)]
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ty:     Type,
	#[format(prefix_ws = Whitespace::set_single)]
	pub value:  Option<ConstantItemValue>,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ConstantItemName {
	Ident(Identifier),
	Underscore(token::Underscore),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstantItemValue {
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::set_single)]
	pub expr: Expression,
}
