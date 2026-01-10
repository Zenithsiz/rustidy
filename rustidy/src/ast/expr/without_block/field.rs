//! Field expression

use {
	super::ExpressionWithoutBlockInner,
	crate::{
		Format,
		ParseRecursive,
		Print,
		ast::{
			expr::{Expression, ExpressionInner},
			ident::Identifier,
			token,
		},
	},
};

/// `FieldExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct FieldExpression {
	pub expr:  Expression,
	pub dot:   token::Dot,
	pub ident: Identifier,
}
