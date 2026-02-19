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
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct CallExpression {
	pub expr:   Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::fmt_remove())]
	pub params: Parenthesized<Option<CallParams>>,
}

/// `MethodCallExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
#[format(args = MethodCallExpressionFmt)]
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
	#[format(args = delimited::fmt_remove())]
	// TODO: Is it fine to remove *all* tags?
	#[format(without_tags)]
	#[format(indent(if_has_tag = FormatTag::InsideChain))]
	pub params:  Parenthesized<Option<CallParams>>,
}

struct MethodCallExpressionFmt;

impl Format<()> for MethodCallExpression {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: rustidy_format::WhitespaceConfig,
		_args: (),
	) -> rustidy_format::FormatOutput {
		let output = self.format(ctx, prefix_ws, MethodCallExpressionFmt);

		match ctx.has_tag(FormatTag::InsideChain) {
			true => output,
			false => match output.len_without_prefix_ws() >= ctx.config().max_chain_len {
				// TODO: Ideally we wouldn't re-format everything here.
				true => ctx.with_tag(FormatTag::InsideChain, |ctx| {
					self.format(ctx, prefix_ws, MethodCallExpressionFmt)
				}),
				false => output,
			},
		}
	}
}

/// `CallParams`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct CallParams(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub  PunctuatedTrailing<Expression, token::Comma>,
);
