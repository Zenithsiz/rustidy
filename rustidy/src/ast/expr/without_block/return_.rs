//! Return

// Imports
use {
	crate::{
		Format,
		Print,
		ast::{expr::Expression, token},
	},
	rustidy_parse::Parse,
};

/// `ReturnExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ReturnExpression {
	pub return_: token::Return,
	// TODO: This needs to be recursive...
	#[parse(skip_if_tag = "skip:OptionalTrailingBlockExpression")]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr:    Option<Expression>,
}
