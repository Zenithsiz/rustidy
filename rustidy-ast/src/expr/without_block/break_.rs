//! Break

// Imports
use {
	crate::expr::Expression,
	ast_literal::{LifetimeOrLabel, token},
	format::{Format, Formattable, WhitespaceFormat},
	parse::{Parse, ParserTag},
	print::Print,
	util::Whitespace,
};

/// `BreakExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct BreakExpression {
	pub continue_: token::Break,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub label:     Option<LifetimeOrLabel>,
	// TODO: Do we need to be parse-recursive here?
	#[parse(skip_if_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:      Option<Expression>,
}
