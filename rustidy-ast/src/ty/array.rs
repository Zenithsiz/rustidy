//! Array type

// Imports
use {
	crate::{expr::Expression, util::Bracketed},
	super::Type,

	ast_util::delimited,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
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
	pub semi: ast_token::Semi,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}
