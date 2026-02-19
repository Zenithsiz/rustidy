//! Constants

// Imports
use {
	crate::{expr::Expression, token, ty::Type},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ConstantItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ConstantItem {
	pub const_: token::Const,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub name:   ConstantItemName,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:     Type,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub value:  Option<ConstantItemValue>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ConstantItemName {
	Ident(Identifier),
	Underscore(token::Underscore),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ConstantItemValue {
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}
