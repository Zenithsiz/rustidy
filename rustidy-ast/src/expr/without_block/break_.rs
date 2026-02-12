//! Break

// Imports
use {
	crate::{expr::Expression, lifetime::LifetimeOrLabel, token},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `BreakExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BreakExpression {
	pub continue_: token::Break,
	#[format(prefix_ws = Whitespace::set_single)]
	pub label:     Option<LifetimeOrLabel>,
	// TODO: Do we need to be parse-recursive here?
	#[parse(skip_if_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub expr:      Option<Expression>,
}
