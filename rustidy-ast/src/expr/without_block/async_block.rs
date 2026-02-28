//! Async block

// Imports
use {
	crate::expr::BlockExpression,

	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `AsyncBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct AsyncBlockExpression {
	pub async_: ast_token::Async,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub move_:  Option<ast_token::Move>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub block:  Box<BlockExpression>,
}
