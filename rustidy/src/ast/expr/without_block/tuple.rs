//! Tuple

// Imports
use {
	crate::ast::{expr::Expression, token, util::Parenthesized},
	rustidy_ast_util::{AtLeast1, at_least},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `TupleExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleExpression(#[format(and_with = Parenthesized::format_remove)] Parenthesized<Option<TupleElements>>);

/// `TupleElements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleElements {
	#[format(and_with = at_least::format(Format::prefix_ws_set_single))]
	pub exprs: AtLeast1<TupleElementsInner>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub last:  Option<Expression>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleElementsInner {
	pub expr:  Expression,
	#[format(before_with = Format::prefix_ws_remove)]
	pub comma: token::Comma,
}
