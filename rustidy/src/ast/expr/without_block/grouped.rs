//! Grouped

// Imports
use {
	crate::ast::{expr::Expression, util::Parenthesized},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `GroupedExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GroupedExpression(#[format(and_with = Parenthesized::format_remove)] Parenthesized<Expression>);
