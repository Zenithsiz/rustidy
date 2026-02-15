//! Async block

// Imports
use {
	crate::{expr::BlockExpression, token},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `AsyncBlockExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AsyncBlockExpression {
	pub async_: token::Async,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub move_:  Option<token::Move>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub block:  Box<BlockExpression>,
}
