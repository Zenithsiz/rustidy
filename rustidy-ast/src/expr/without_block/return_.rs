//! Return

// Imports
use {
	crate::{expr::Expression, token},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ReturnExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ReturnExpression {
	pub return_: token::Return,
	// TODO: This needs to be recursive...
	#[parse(skip_if_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:    Option<Expression>,
}
