//! Return

// Imports
use {
	crate::{expr::Expression, token},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `ReturnExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ReturnExpression {
	pub return_: token::Return,
	// TODO: This needs to be recursive...
	#[parse(skip_if_tag = SkipOptionalTrailingBlockExpression)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr:    Option<Expression>,
}
