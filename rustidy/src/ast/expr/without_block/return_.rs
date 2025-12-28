//! Return

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{expr::Expression, token},
};

/// `ReturnExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ReturnExpression {
	pub return_: token::Return,
	// TODO: This needs to be recursive...
	pub expr:    Option<Box<Expression>>,
}
