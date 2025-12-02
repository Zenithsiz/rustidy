//! Tuple indexing expression

use {
	super::{Expression, ExpressionWithoutBlockInner, literal::TupleIndex},
	crate::{Format, ParseRecursive, Print, ast::token},
};

/// `TupleIndexingExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct TupleIndexingExpression {
	pub expr:  Box<Expression>,
	pub dot:   token::Dot,
	pub ident: TupleIndex,
}
