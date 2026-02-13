//! Tuple

// Imports
use {
	crate::{expr::Expression, token, util::Parenthesized},
	rustidy_ast_util::{AtLeast1, at_least, delimited},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `TupleExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleExpression(
	#[format(args = delimited::FmtArgs::remove((), (), ()))] Parenthesized<Option<TupleElements>>,
);

/// `TupleElements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleElements {
	#[format(args = at_least::FmtArgs::from_prefix_ws(Whitespace::set_single))]
	pub exprs: AtLeast1<TupleElementsInner>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub last:  Option<Expression>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleElementsInner {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::remove)]
	pub comma: token::Comma,
}
