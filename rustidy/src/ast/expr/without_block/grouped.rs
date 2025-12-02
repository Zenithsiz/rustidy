//! Grouped

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{delimited::Parenthesized, expr::Expression},
};

/// `GroupedExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GroupedExpression(Parenthesized<Box<Expression>>);
