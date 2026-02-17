//! Field expression

use {
	super::ExpressionWithoutBlockInner,
	crate::{
		expr::{Expression, ExpressionInner},
		token,
	},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, FormatTag, Formattable, WhitespaceFormat},
	rustidy_parse::ParseRecursive,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `FieldExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
#[format(args = FieldExpressionFmt)]
pub struct FieldExpression {
	pub expr:  Expression,
	#[format(prefix_ws = match ctx.has_tag(FormatTag::InsideChain) {
		true => Whitespace::NEXT_INDENT,
		false => Whitespace::REMOVE,
	})]
	pub dot:   token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub ident: Identifier,
}

struct FieldExpressionFmt;

impl Format<()> for FieldExpression {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: rustidy_format::WhitespaceConfig,
		(): &mut (),
	) -> rustidy_format::FormatOutput {
		let output = self.format(ctx, prefix_ws, &mut FieldExpressionFmt);

		match ctx.has_tag(FormatTag::InsideChain) {
			true => output,
			false => match output.len_without_prefix_ws() >= ctx.config().max_chain_len {
				// TODO: Ideally we wouldn't re-format everything here.
				true => ctx.with_tag(FormatTag::InsideChain, |ctx| {
					self.format(ctx, prefix_ws, &mut FieldExpressionFmt)
				}),
				false => output,
			},
		}
	}
}
