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
#[format(with_tag(
	tag = FormatTag::InsideChain,
	if = self.len(ctx, true) >= ctx.config().max_chain_len,
	skip_if_has_tag = true
))]
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
