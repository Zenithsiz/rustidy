//! Grouped

// Imports
use {
	crate::{expr::Expression, util::Parenthesized},
	rustidy_ast_util::delimited,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `GroupedExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GroupedExpression(#[format(args = delimited::FmtArgs::remove((), (), ()))] Parenthesized<Expression>);
