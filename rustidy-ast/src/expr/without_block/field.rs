//! Field expression

use {
	super::ExpressionWithoutBlockInner,
	crate::{
		expr::{Expression, ExpressionInner},
		token,
	},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, FormatTag},
	rustidy_parse::ParseRecursive,
	rustidy_print::Print,
};

/// `FieldExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
#[format(with_tag(
	tag = InsideChain,
	if = self.len(ctx, true) >= ctx.config().max_chain_len,
	skip_if_has_tag = true
))]
pub struct FieldExpression {
	pub expr:  Expression,
	#[format(before_with = match ctx.has_tag(FormatTag::InsideChain) {
		true => move |dot, ctx| Format::prefix_ws_set_indent(dot, ctx, 1, false),
		false => Format::prefix_ws_remove,
	})]
	pub dot:   token::Dot,
	#[format(before_with = Format::prefix_ws_remove)]
	pub ident: Identifier,
}
