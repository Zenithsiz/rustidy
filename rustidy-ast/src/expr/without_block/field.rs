//! Field expression

use {
	crate::expr::{Expression, ExpressionInner},
	super::ExpressionWithoutBlockInner,
	ast_literal::Identifier,
	format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
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
	#[format(indent(if_ = args.indent))]
	#[format(prefix_ws = match args.indent {
		true => Whitespace::INDENT,
		false => Whitespace::REMOVE,
	})]
	pub dot:   ast_token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub ident: Identifier,
}

impl FieldExpression {
	fn format_inside_chain(
		&mut self,
		ctx: &mut format::Context,
		prefix_ws: WhitespaceConfig,
		indent: bool
	) -> FormatOutput {
		self
			.format(ctx, prefix_ws, FieldExpressionFmt { indent })
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

struct FieldExpressionFmt {
	indent: bool,
}

impl Format<WhitespaceConfig, ()> for FieldExpression {
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
