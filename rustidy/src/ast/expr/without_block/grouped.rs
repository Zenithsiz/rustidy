//! Grouped

// Imports
use {
	crate::{
		Format,
		ast::{delimited::Parenthesized, expr::Expression},
	},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `GroupedExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GroupedExpression(#[format(and_with = Parenthesized::format_remove)] Parenthesized<Expression>);
