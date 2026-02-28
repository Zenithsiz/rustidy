//! Async block

// Imports
use {
	crate::expr::BlockExpression,
	ast_literal::token,
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
	pub async_: token::Async,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub move_:  Option<token::Move>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub block:  Box<BlockExpression>,
}
