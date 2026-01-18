//! Range


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
	rustidy_parse::{Parse, ParseRecursive},
};

/// `RangeExpression`
#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:RangeExpr")]
pub struct RangeExpr {
	pub lhs:     Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	pub dot_dot: token::DotDot,
	#[format(and_with = Format::prefix_ws_remove)]
	pub rhs:     Expression,
}

/// `RangeFromExpr`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "left")]
#[parse_recursive(skip_if_tag = "skip:RangeFromExpr")]
pub struct RangeFromExpr {
	pub lhs:     Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	pub dot_dot: token::DotDot,
}

/// `RangeToExpr`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "right")]
pub struct RangeToExpr {
	pub dot_dot: token::DotDot,
	#[format(and_with = Format::prefix_ws_remove)]
	pub rhs:     Expression,
}

/// `RangeFullExpr`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RangeFullExpr(token::DotDot);


/// `RangeInclusiveExpr`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:RangeInclusiveExpr")]
pub struct RangeInclusiveExpr {
	pub lhs:        Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	pub dot_dot_eq: token::DotDotEq,
	#[format(and_with = Format::prefix_ws_remove)]
	pub rhs:        Expression,
}

/// `RangeToInclusiveExpr`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "right")]
pub struct RangeToInclusiveExpr {
	pub dot_dot_eq: token::DotDotEq,
	#[format(and_with = Format::prefix_ws_remove)]
	pub rhs:        Expression,
}
