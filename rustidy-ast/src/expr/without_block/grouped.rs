//! Grouped

// Imports
use {
	crate::{expr::Expression, util::Parenthesized},
	rustidy_ast_util::delimited,
	rustidy_format::{Format, Formattable},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `GroupedExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct GroupedExpression(#[format(args = delimited::FmtRemove)]
Parenthesized<Expression>);
