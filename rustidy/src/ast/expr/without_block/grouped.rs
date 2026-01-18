//! Grouped

// Imports
use {
	crate::{
		Format,
		Print,
		ast::{delimited::Parenthesized, expr::Expression},
	},
	rustidy_parse::Parse,
};

/// `GroupedExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GroupedExpression(#[format(and_with = Parenthesized::format_remove)] Parenthesized<Expression>);
