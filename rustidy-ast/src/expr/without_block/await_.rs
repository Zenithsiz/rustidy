//! Await

// Imports
use {
	crate::expr::{Expression, ExpressionInner},
	super::ExpressionWithoutBlockInner,
	rustidy_ast_literal::token,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::ParseRecursive,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `AwaitExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct AwaitExpression {
	pub expr:   Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot:    token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub await_: token::Await,
}
