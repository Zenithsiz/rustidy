//! Grouped

// Imports
use {
	crate::{expr::Expression, util::Parenthesized},
	ast_util::delimited,
	format::{Format, Formattable},
	parse::Parse,
	print::Print,
};

/// `GroupedExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct GroupedExpression(#[format(args = delimited::FmtRemove)] Parenthesized<Expression>);
