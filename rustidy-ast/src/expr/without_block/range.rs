//! Range


// Imports
use {
	crate::expr::{Expression, ExpressionInner},
	super::ExpressionWithoutBlockInner,
	rustidy_ast_literal::token,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, ParseRecursive, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `RangeExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
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
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = ParserTag::SkipRangeExpr)]
pub struct RangeExpr {
	pub lhs:     Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot_dot: token::DotDot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub rhs:     Expression,
}

/// `RangeFromExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "left")]
#[parse_recursive(skip_if_tag = ParserTag::SkipRangeFromExpr)]
pub struct RangeFromExpr {
	pub lhs:     Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot_dot: token::DotDot,
}

/// `RangeToExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "right")]
pub struct RangeToExpr {
	pub dot_dot: token::DotDot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub rhs:     Expression,
}

/// `RangeFullExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RangeFullExpr(token::DotDot);


/// `RangeInclusiveExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = ParserTag::SkipRangeInclusiveExpr)]
pub struct RangeInclusiveExpr {
	pub lhs:        Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot_dot_eq: token::DotDotEq,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub rhs:        Expression,
}

/// `RangeToInclusiveExpr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = RangeExpression)]
#[parse_recursive(kind = "right")]
pub struct RangeToInclusiveExpr {
	pub dot_dot_eq: token::DotDotEq,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub rhs:        Expression,
}
