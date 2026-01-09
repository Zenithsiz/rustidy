//! Operator expression

// Imports
use {
	super::{Expression, ExpressionWithoutBlockInner, Parse},
	crate::{
		Format,
		ParseRecursive,
		Print,
		ast::{token, ty::TypeNoBounds},
	},
};

/// `OperatorExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
pub enum OperatorExpression {
	#[parse_recursive(recursive)]
	LazyBoolean(LazyBooleanExpression),
	#[parse_recursive(recursive)]
	CompoundAssignment(CompoundAssignmentExpression),

	#[parse_recursive(recursive)]
	Borrow(BorrowExpression),
	#[parse_recursive(recursive)]
	Dereference(DereferenceExpression),
	#[parse_recursive(recursive)]
	TryPropagation(TryPropagationExpression),
	#[parse_recursive(recursive)]
	Negation(NegationExpression),
	#[parse_recursive(recursive)]
	ArithmeticOrLogical(ArithmeticOrLogicalExpression),
	#[parse_recursive(recursive)]
	Comparison(ComparisonExpression),
	#[parse_recursive(recursive)]
	TypeCast(TypeCastExpression),
	#[parse_recursive(recursive)]
	Assignment(AssignmentExpression),
}

/// `ErrorPropagationExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "left")]
pub struct TryPropagationExpression {
	pub expr:     Box<Expression>,
	pub question: token::Question,
}

/// `BorrowExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "right")]
pub struct BorrowExpression {
	pub ref_: BorrowExpressionKindRef,
	pub rest: Option<BorrowExpressionKindRest>,
	pub expr: Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum BorrowExpressionKindRef {
	And(token::And),
	AndAnd(token::AndAnd),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum BorrowExpressionKindRest {
	Mut(token::Mut),
	RawConst((token::Raw, token::Const)),
	RawMut((token::Raw, token::Mut)),
}

/// `DereferenceExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "right")]
pub struct DereferenceExpression {
	pub star: token::Star,
	pub expr: Box<Expression>,
}

/// `NegationExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "right")]
pub struct NegationExpression {
	pub token: NegationExpressionToken,
	pub expr:  Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum NegationExpressionToken {
	Minus(token::Minus),
	Not(token::Not),
}

/// `ArithmeticOrLogicalExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
pub struct ArithmeticOrLogicalExpression {
	pub lhs: Box<Expression>,
	pub op:  ArithmeticOrLogicalExpressionOp,
	pub rhs: Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ArithmeticOrLogicalExpressionOp {
	Plus(token::Plus),
	Minus(token::Minus),
	Star(token::Star),
	Div(token::Slash),
	Percent(token::Percent),
	And(token::And),
	Or(token::Or),
	Caret(token::Caret),
	Shl(token::Shl),
	Shr(token::Shr),
}

/// `ComparisonExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
pub struct ComparisonExpression {
	pub lhs: Box<Expression>,
	pub op:  ComparisonExpressionOp,
	pub rhs: Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ComparisonExpressionOp {
	EqEq(token::EqEq),
	Ne(token::Ne),
	Ge(token::Ge),
	Le(token::Le),
	Gt(token::Gt),
	Lt(token::Lt),
}

/// `LazyBooleanExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:LazyBooleanExpression")]
pub struct LazyBooleanExpression {
	pub lhs: Box<Expression>,
	pub op:  LazyBooleanExpressionOp,
	pub rhs: Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LazyBooleanExpressionOp {
	Or(token::OrOr),
	And(token::AndAnd),
}

/// `TypeCastExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "left")]
pub struct TypeCastExpression {
	pub lhs: Box<Expression>,
	pub as_: token::As,
	pub ty:  TypeNoBounds,
}

/// `AssignmentExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:AssignmentExpression")]
pub struct AssignmentExpression {
	pub lhs: Box<Expression>,
	pub eq:  token::Eq,
	pub rhs: Box<Expression>,
}

/// `CompoundAssignmentExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:CompoundAssignmentExpression")]
pub struct CompoundAssignmentExpression {
	pub lhs: Box<Expression>,
	pub op:  CompoundAssignmentExpressionOp,
	pub rhs: Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum CompoundAssignmentExpressionOp {
	Plus(token::PlusEq),
	Minus(token::MinusEq),
	Star(token::StarEq),
	Div(token::SlashEq),
	Percent(token::PercentEq),
	And(token::AndEq),
	Or(token::OrEq),
	Caret(token::CaretEq),
	Shl(token::ShlEq),
	Shr(token::ShrEq),
}
