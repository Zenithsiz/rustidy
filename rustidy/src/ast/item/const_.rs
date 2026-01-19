//! Constants

// Imports
use {
	crate::ast::{expr::Expression, token, ty::Type}, rustidy_ast_util::Identifier, rustidy_format::Format, rustidy_parse::Parse, rustidy_print::Print
};

/// `ConstantItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstantItem {
	pub const_: token::Const,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub name:   ConstantItemName,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon:  token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:     Type,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub value:  Option<ConstantItemValue>,
	#[format(and_with = Format::prefix_ws_remove)]
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
}
