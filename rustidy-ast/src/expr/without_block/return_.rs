//! Return

// Imports
use {
	crate::expr::Expression,

	format::{Format, Formattable, WhitespaceFormat},
	parse::{Parse, ParserTag},
	print::Print,
	util::Whitespace,
};

/// `ReturnExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ReturnExpression {
	pub return_: ast_token::Return,
	// TODO: This needs to be recursive...
	#[parse(skip_if_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:    Option<Expression>,
}
