//! Constants

// Imports
use {
	crate::{expr::Expression, ty::Type},
	ast_literal::Identifier,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `ConstantItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ConstantItem {
	pub const_: ast_token::Const,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub name:   ConstantItemName,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:  ast_token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:     Type,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub value:  Option<ConstantItemValue>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   ast_token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ConstantItemName {
	Ident(Identifier),
	Underscore(ast_token::Underscore),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ConstantItemValue {
	pub eq:   ast_token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}
