//! Method call expression

use {
	super::{Expression, ExpressionWithoutBlockInner, path::PathExprSegment},
	crate::{
		Format,
		Parse,
		ParseRecursive,
		Print,
		ast::{delimited::Parenthesized, punct::PunctuatedTrailing, token},
	},
};

/// `CallExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct CallExpression {
	pub expr:   Box<Expression>,
	pub params: Parenthesized<Option<CallParams>>,
}

/// `MethodCallExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct MethodCallExpression {
	pub expr:    Box<Expression>,
	pub dot:     token::Dot,
	pub segment: PathExprSegment,
	pub params:  Parenthesized<Option<CallParams>>,
}

/// `CallParams`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct CallParams {
	params: PunctuatedTrailing<Box<Expression>, token::Comma>,
}
