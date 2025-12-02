//! Break

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{expr::Expression, lifetime::LifetimeOrLabel, token},
};

/// `BreakExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BreakExpression {
	continue_: token::Break,
	label:     Option<LifetimeOrLabel>,
	// TODO: Do we need to be parse-recursive here?
	expr:      Option<Box<Expression>>,
}
