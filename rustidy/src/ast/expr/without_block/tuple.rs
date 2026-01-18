//! Tuple

// Imports
use {
	crate::{
		Format,
		Print,
		ast::{
			at_least::{self, AtLeast1},
			delimited::Parenthesized,
			expr::Expression,
			token,
		},
	},
	rustidy_parse::Parse,
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub last:  Option<Expression>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleElementsInner {
	pub expr:  Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	pub comma: token::Comma,
}
