//! Method call expression

use {
	crate::{
		expr::{Expression, ExpressionInner},
		util::{FmtSingleOrIndent, Parenthesized},
	},
	super::{ExpressionWithoutBlockInner, path::PathExprSegment},

	ast_util::{PunctuatedTrailing, delimited, punct},
	format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	parse::{Parse, ParseRecursive},
	print::Print,
	util::Whitespace,
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
	#[format(without_tag = format::tag::InsideChain)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::fmt_remove_or_indent_if_non_blank(
		50,
		FmtSingleOrIndent::Single,
		FmtSingleOrIndent::Indent
	))]
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
	#[format(indent(if_ = ctx.has_tag::<format::tag::InsideChain>()))]
	#[format(prefix_ws = match ctx.has_tag::<format::tag::InsideChain>() {
		true => Whitespace::INDENT,
		false => Whitespace::REMOVE,
	})]
	pub dot:     ast_token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub segment: PathExprSegment,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(without_tag = format::tag::InsideChain)]
	#[format(indent(if_ = ctx.has_tag::<format::tag::InsideChain>()))]
	#[format(args = delimited::fmt_remove_or_indent_if_non_blank(
		50,
		FmtSingleOrIndent::Single,
		FmtSingleOrIndent::Indent
	))]
	pub params:  Parenthesized<Option<CallParams>>,
}

struct MethodCallExpressionFmt;

impl Format<WhitespaceConfig, ()> for MethodCallExpression {
	fn format(
		&mut self,
		ctx: &mut format::Context,
		prefix_ws: WhitespaceConfig,
		_args: ()
	) -> FormatOutput {
		let output = self
			.format(ctx, prefix_ws, MethodCallExpressionFmt);

		match ctx.has_tag::<format::tag::InsideChain>() {
			true => output,
			false => match output.len_non_multiline_ws() >= ctx.config().max_chain_len {
				// TODO: Ideally we wouldn't re-format everything here.
				true => ctx
					.with_tag::<format::tag::InsideChain, _>(|ctx| {
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
#[format(args(ty = "FmtSingleOrIndent"))]
pub struct CallParams(
	#[format(args = punct::fmt(args.prefix_ws(), Whitespace::REMOVE))]
	pub PunctuatedTrailing<Expression, ast_token::Comma>,
);
