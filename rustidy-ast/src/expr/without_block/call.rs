//! Method call expression

use {
	super::{ExpressionWithoutBlockInner, path::PathExprSegment},
	crate::{
		expr::{Expression, ExpressionInner},
		token,
		util::Parenthesized,
	},
	rustidy_ast_util::{PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, FormatTag, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, ParseRecursive},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `CallExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct CallExpression {
	pub expr:   Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub params: Parenthesized<Option<CallParams>>,
}

/// `MethodCallExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
#[format(with_tag(
	tag = FormatTag::InsideChain,
	if = self.len(ctx, true) >= ctx.config().max_chain_len,
	skip_if_has_tag = true
))]
pub struct MethodCallExpression {
	pub expr:    Expression,
	#[format(prefix_ws = match ctx.has_tag(FormatTag::InsideChain) {
		true => Whitespace::NEXT_INDENT,
		false => Whitespace::REMOVE,
	})]
	pub dot:     token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub segment: PathExprSegment,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	// TODO: Is it fine to remove *all* tags?
	#[format(without_tags)]
	#[format(indent(if_has_tag = FormatTag::InsideChain))]
	pub params:  Parenthesized<Option<CallParams>>,
}

/// `CallParams`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct CallParams(
	#[format(args = punct::args(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub  PunctuatedTrailing<Expression, token::Comma>,
);
