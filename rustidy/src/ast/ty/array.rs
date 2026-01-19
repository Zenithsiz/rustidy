//! Array type

// Imports
use {
	super::Type,
	crate::ast::{delimited::Bracketed, expr::Expression, token},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `ArrayType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayType(#[format(and_with = Bracketed::format_remove)] Bracketed<ArrayTypeInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayTypeInner {
	pub ty:   Box<Type>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub semi: token::Semi,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
}
