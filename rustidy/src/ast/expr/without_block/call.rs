//! Method call expression

use {
	super::{ExpressionWithoutBlockInner, path::PathExprSegment},
	crate::{
		Format,
		Parse,
		ParseRecursive,
		Print,
		ast::{
			delimited::Parenthesized,
			expr::{Expression, ExpressionInner},
			punct::PunctuatedTrailing,
			token,
		},
	},
};

/// `CallExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct CallExpression {
	pub expr:   Expression,
	pub params: Parenthesized<Option<CallParams>>,
}

/// `MethodCallExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "left")]
pub struct MethodCallExpression {
	pub expr:    Expression,
	pub dot:     token::Dot,
	pub segment: PathExprSegment,
	pub params:  Parenthesized<Option<CallParams>>,
}

/// `CallParams`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct CallParams(pub PunctuatedTrailing<Expression, token::Comma>);
