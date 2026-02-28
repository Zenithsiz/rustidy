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
	#[format(indent(if_ = args.indent))]
	#[format(prefix_ws = match args.indent {
		true => Whitespace::INDENT,
		false => Whitespace::REMOVE,
	})]
	pub dot:     ast_token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub segment: PathExprSegment,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(without_tag = format::tag::InsideChain)]
	#[format(indent(if_ = args.indent))]
	#[format(args = delimited::fmt_remove_or_indent_if_non_blank(
		50,
		FmtSingleOrIndent::Single,
		FmtSingleOrIndent::Indent
	))]
	pub params:  Parenthesized<Option<CallParams>>,
}

impl MethodCallExpression {
	fn format_inside_chain(
		&mut self,
		ctx: &mut format::Context,
		prefix_ws: WhitespaceConfig,
		indent: bool
	) -> FormatOutput {
		self.format(
			ctx,
			prefix_ws,
			MethodCallExpressionFmt { indent }
		)
	}

	fn format_outside_chain(
		&mut self,
		ctx: &mut format::Context,
		prefix_ws: WhitespaceConfig,
		indent: bool
	) -> FormatOutput {
		ctx.with_tag_with::<format::tag::InsideChain, _>(
			format::tag::InsideChainData { indent },
			|ctx| self
				.format_inside_chain(ctx, prefix_ws, indent)
		)
	}
}

struct MethodCallExpressionFmt {
	indent: bool,
}

impl Format<WhitespaceConfig, ()> for MethodCallExpression {
	fn format(
		&mut self,
		ctx: &mut format::Context,
		prefix_ws: WhitespaceConfig,
		_args: ()
	) -> FormatOutput {
		match ctx.tag::<format::tag::InsideChain>() {
			Some(&format::tag::InsideChainData { indent }) => self.format_inside_chain(ctx, prefix_ws, indent),
			None => {
				let singleline_output = self.format_outside_chain(ctx, prefix_ws, false);
				match singleline_output.len_non_multiline_ws() >= ctx.config().max_chain_len {
					true => self.format_outside_chain(ctx, prefix_ws, true),
					false => singleline_output,
				}
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
