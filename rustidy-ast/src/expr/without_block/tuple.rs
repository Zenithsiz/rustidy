//! Tuple

// Imports
use {
	crate::{expr::Expression, token, util::Parenthesized},
	rustidy_ast_util::{AtLeast1, at_least, delimited},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `TupleExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleExpression(#[format(args = delimited::fmt_remove())]
Parenthesized<Option<TupleElements>>);

/// `TupleElements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleElements {
	#[format(args = at_least::fmt_prefix_ws(Whitespace::SINGLE))]
	pub exprs: AtLeast1<TupleElementsInner>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub last:  Option<Expression>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleElementsInner {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub comma: token::Comma,
}
