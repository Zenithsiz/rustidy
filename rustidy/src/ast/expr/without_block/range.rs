//! Range


// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::{
		Format,
		Parse,
		ParseRecursive,
		Print,
		ast::{
			expr::{Expression, ExpressionInner},
			token,
		},
	},
};

/// `RangeExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
pub enum RangeExpression {
	#[parse_recursive(recursive)]
	Inclusive(RangeInclusiveExpr),
	#[parse_recursive(recursive)]
	ToInclusive(RangeToInclusiveExpr),

	#[parse_recursive(recursive)]
	Both(RangeExpr),
	#[parse_recursive(recursive)]
	From(RangeFromExpr),
	#[parse_recursive(recursive)]
	To(RangeToExpr),
	Full(RangeFullExpr),
}

/// `RangeExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:RangeExpr")]
pub struct RangeExpr {
	pub lhs:     Expression,
	pub dot_dot: token::DotDot,
	pub rhs:     Expression,
}

/// `RangeFromExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "left")]
#[parse_recursive(skip_if_tag = "skip:RangeFromExpr")]
pub struct RangeFromExpr {
	pub lhs:     Expression,
	pub dot_dot: token::DotDot,
}

/// `RangeToExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "right")]
pub struct RangeToExpr {
	pub dot_dot: token::DotDot,
	pub rhs:     Expression,
}

/// `RangeFullExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeFullExpr(token::DotDot);


/// `RangeInclusiveExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:RangeInclusiveExpr")]
pub struct RangeInclusiveExpr {
	pub lhs:        Expression,
	pub dot_dot_eq: token::DotDotEq,
	pub rhs:        Expression,
}

/// `RangeToInclusiveExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "right")]
pub struct RangeToInclusiveExpr {
	pub dot_dot_eq: token::DotDotEq,
	pub rhs:        Expression,
}
