//! Async block

// Imports
use {
	crate::{
		Format,
		ast::{expr::BlockExpression, token},
	},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `AsyncBlockExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AsyncBlockExpression {
	pub async_: token::Async,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub move_:  Option<token::Move>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub block:  Box<BlockExpression>,
}
