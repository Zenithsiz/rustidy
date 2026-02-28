//! Array type

// Imports
use {
	crate::{expr::Expression, util::Bracketed},
	super::Type,
	rustidy_ast_literal::token,
	rustidy_ast_util::delimited,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ArrayType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ArrayType(#[format(args = delimited::FmtRemove)] Bracketed<ArrayTypeInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ArrayTypeInner {
	pub ty:   Box<Type>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi: token::Semi,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}
