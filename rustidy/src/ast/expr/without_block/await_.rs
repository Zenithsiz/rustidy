//! Await

// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::{
		Format,
		Print,
		ast::{
			expr::{Expression, ExpressionInner},
			token,
		},
	},
	rustidy_parse::ParseRecursive,
};

/// `AwaitExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct AwaitExpression {
	pub expr:   Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	pub dot:    token::Dot,
	#[format(and_with = Format::prefix_ws_remove)]
	pub await_: token::Await,
}
