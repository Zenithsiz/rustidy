//! Method call expression

use {
	super::{ExpressionWithoutBlockInner, path::PathExprSegment},
	crate::{expr::{Expression, ExpressionInner}, token, util::Parenthesized},
	rustidy_ast_util::{PunctuatedTrailing, delimited, punct},
	rustidy_format::{
		Format,
		FormatOutput,
		FormatTag,
		Formattable,
		WhitespaceConfig,
		WhitespaceFormat,
	},
	rustidy_parse::{Parse, ParseRecursive},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `CallExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct CallExpression {
	pub expr:   Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtRemove)]
	pub params: Parenthesized<Option<CallParams>>,
}

/// `MethodCallExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
#[format(args = MethodCallExpressionFmt)]
pub struct MethodCallExpression {
	pub expr:    Expression,
	#[format(indent(if_has_tag = FormatTag::InsideChain))]
	#[format(
		prefix_ws = match ctx.has_tag(FormatTag::InsideChain) {
			true => Whitespace::INDENT,
			false => Whitespace::REMOVE,
		}
	)]
	pub dot:     token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub segment: PathExprSegment,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtRemove)]
	// TODO: Is it fine to remove *all* tags?
	#[format(without_tags)]
	#[format(indent(if_has_tag = FormatTag::InsideChain))]
	pub params:  Parenthesized<Option<CallParams>>,
}

struct MethodCallExpressionFmt;

impl Format<WhitespaceConfig, ()> for MethodCallExpression {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: WhitespaceConfig, _args: ()) -> FormatOutput {
		let output = self
			.format(ctx, prefix_ws, MethodCallExpressionFmt);

		match ctx.has_tag(FormatTag::InsideChain) {
			true => output,
			false => match output.len_without_prefix_ws() >= ctx.config().max_chain_len {
				// TODO: Ideally we wouldn't re-format everything here.
				true => ctx
					.with_tag(FormatTag::InsideChain, |ctx| {
						self
							.format(ctx, prefix_ws, MethodCallExpressionFmt)
					}),
				false => output,
			},
		}
	}
}

/// `CallParams`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct CallParams(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub PunctuatedTrailing<Expression, token::Comma>,
);
