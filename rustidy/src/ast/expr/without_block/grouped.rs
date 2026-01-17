//! Grouped

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{delimited::Parenthesized, expr::Expression},
};

/// `GroupedExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GroupedExpression(#[format(and_with = Parenthesized::format_remove)] Parenthesized<Expression>);
