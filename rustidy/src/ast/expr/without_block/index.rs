//! Index

// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::{
		Format,
		ParseRecursive,
		Print,
		ast::{
			delimited::Bracketed,
			expr::{Expression, ExpressionInner},
		},
	},
};

/// `IndexExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct IndexExpression {
	pub expr:  Expression,
	pub index: Bracketed<Expression>,
}
