//! Tuple indexing expression

use {
	crate::expr::{Expression, ExpressionInner},
	super::{ExpressionWithoutBlockInner, TupleIndex},

	format::{Format, Formattable, WhitespaceFormat},
	parse::ParseRecursive,
	print::Print,
	util::Whitespace,
};

/// `TupleIndexingExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct TupleIndexingExpression {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot:   ast_token::Dot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub ident: TupleIndex,
}
