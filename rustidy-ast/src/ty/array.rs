//! Array type

// Imports
use {
	super::Type,
	crate::{expr::Expression, token, util::Bracketed},
	rustidy_ast_util::delimited,
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ArrayType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayType(#[format(args = delimited::FmtArgs::remove((), (), ()))] Bracketed<ArrayTypeInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayTypeInner {
	pub ty:   Box<Type>,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi: token::Semi,
	#[format(prefix_ws = Whitespace::set_single)]
	pub expr: Expression,
}
