//! Field expression

use {
	crate::expr::{Expression, ExpressionInner},
	super::ExpressionWithoutBlockInner,
	ast_literal::Identifier,
	format::{
		Format,
		FormatOutput,
		FormatTag,
		Formattable,
		WhitespaceConfig,
		WhitespaceFormat,
	},
	parse::ParseRecursive,
	print::Print,
	util::Whitespace,
};

/// `FieldExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
#[format(args = FieldExpressionFmt)]
pub struct FieldExpression {
	pub expr:  Expression,
	#[format(indent(if_ = ctx.has_tag(FormatTag::InsideChain)))]
	#[format(prefix_ws = match ctx.has_tag(FormatTag::InsideChain) {
		true => Whitespace::INDENT,
		false => Whitespace::REMOVE,
	})]
	pub dot:   ast_token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub ident: Identifier,
}

struct FieldExpressionFmt;

impl Format<WhitespaceConfig, ()> for FieldExpression {
	fn format(
		&mut self,
		ctx: &mut format::Context,
		prefix_ws: WhitespaceConfig,
		_args: ()
	) -> FormatOutput {
		let output = self.format(ctx, prefix_ws, FieldExpressionFmt);

		match ctx.has_tag(FormatTag::InsideChain) {
			true => output,
			false => match output.len_non_multiline_ws() >= ctx.config().max_chain_len {
				// TODO: Ideally we wouldn't re-format everything here.
				true => ctx
					.with_tag(FormatTag::InsideChain, |ctx| {
						self
							.format(ctx, prefix_ws, FieldExpressionFmt)
					}),
				false => output,
			},
		}
	}
}
