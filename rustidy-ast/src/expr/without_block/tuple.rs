//! Tuple

// Imports
use {
	crate::{expr::Expression, util::Parenthesized},
	ast_literal::token,
	ast_util::{AtLeast1, at_least, delimited},
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `TupleExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleExpression(#[format(args = delimited::FmtRemove)] Parenthesized<Option<TupleElements>>);

/// `TupleElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleElements {
	#[format(args = at_least::fmt_prefix_ws(Whitespace::SINGLE))]
	pub exprs: AtLeast1<TupleElementsInner>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub last:  Option<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleElementsInner {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub comma: token::Comma,
}
