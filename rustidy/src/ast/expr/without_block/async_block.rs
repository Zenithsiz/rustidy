//! Async block

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{expr::BlockExpression, token},
};

/// `AsyncBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AsyncBlockExpression {
	async_: token::Async,
	move_:  Option<token::Move>,
	block:  Box<BlockExpression>,
}
