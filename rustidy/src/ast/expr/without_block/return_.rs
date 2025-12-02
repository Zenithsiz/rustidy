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
	return_: token::Return,
	// TODO: This needs to be recursive...
	expr:    Option<Box<Expression>>,
}
