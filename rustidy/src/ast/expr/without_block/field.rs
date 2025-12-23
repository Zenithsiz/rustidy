//! Field expression

use {
	super::{Expression, ExpressionWithoutBlockInner},
	crate::{
		Format,
		ParseRecursive,
		Print,
		ast::{ident::Identifier, token},
	},
};

/// `FieldExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct FieldExpression {
	pub expr:  Box<Expression>,
	pub dot:   token::Dot,
	pub ident: Identifier,
}
