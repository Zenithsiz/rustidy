//! Tuple

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{at_least::AtLeast1, delimited::Parenthesized, expr::Expression, token},
};

/// `TupleExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleExpression(Parenthesized<Option<TupleElements>>);

/// `TupleElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleElements {
	pub exprs: AtLeast1<(Box<Expression>, token::Comma)>,
	pub last:  Option<Box<Expression>>,
}
