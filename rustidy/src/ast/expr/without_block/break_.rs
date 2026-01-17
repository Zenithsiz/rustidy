//! Break

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{expr::Expression, lifetime::LifetimeOrLabel, token},
};

/// `BreakExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BreakExpression {
	pub continue_: token::Break,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub label:     Option<LifetimeOrLabel>,
	// TODO: Do we need to be parse-recursive here?
	#[parse(skip_if_tag = "skip:OptionalTrailingBlockExpression")]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr:      Option<Expression>,
}
