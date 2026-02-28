//! Await

// Imports
use {
	crate::expr::{Expression, ExpressionInner},
	super::ExpressionWithoutBlockInner,

	format::{Format, Formattable, WhitespaceFormat},
	parse::ParseRecursive,
	print::Print,
	util::Whitespace,
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
	pub dot:    ast_token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub await_: ast_token::Await,
}
