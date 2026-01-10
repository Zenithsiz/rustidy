//! Await

// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::{
		Format,
		ParseRecursive,
		Print,
		ast::{
			expr::{Expression, ExpressionInner},
			token,
		},
	},
};

/// `AwaitExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct AwaitExpression {
	pub expr:   Expression,
	pub dot:    token::Dot,
	pub await_: token::Await,
}
