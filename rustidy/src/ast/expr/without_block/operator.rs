//! Operator expression

// Imports
use {
	super::{ExpressionWithoutBlockInner, Parse},
	crate::ast::{
		expr::{Expression, ExpressionInner},
		token,
		ty::TypeNoBounds,
	},
	rustidy_format::Format,
	rustidy_parse::ParseRecursive,
	rustidy_print::Print,
};

/// `OperatorExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "left")]
pub struct TryPropagationExpression {
	pub expr:     Expression,
	#[format(and_with = Format::prefix_ws_remove)]
	pub question: token::Question,
}

/// `BorrowExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "right")]
pub struct BorrowExpression {
	pub ref_: BorrowExpressionKindRef,
	#[format(and_with = Format::prefix_ws_remove)]
	pub rest: Option<BorrowExpressionKindRest>,
	#[format(and_with = match self.rest.is_some() {
		true => Format::prefix_ws_set_single,
		false => Format::prefix_ws_remove,
	})]
	pub expr: Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum BorrowExpressionKindRef {
	And(token::And),
	AndAnd(token::AndAnd),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum BorrowExpressionKindRest {
	Mut(token::Mut),
	RawConst((token::Raw, token::Const)),
	RawMut((token::Raw, token::Mut)),
}

/// `DereferenceExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "right")]
pub struct DereferenceExpression {
	pub star: token::Star,
	#[format(and_with = Format::prefix_ws_remove)]
	pub expr: Expression,
}

/// `NegationExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "right")]
pub struct NegationExpression {
	pub token: NegationExpressionToken,
	#[format(and_with = Format::prefix_ws_remove)]
	pub expr:  Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum NegationExpressionToken {
	Minus(token::Minus),
	Not(token::Not),
}

/// `ArithmeticOrLogicalExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
pub struct ArithmeticOrLogicalExpression {
	pub lhs: Expression,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub op:  ArithmeticOrLogicalExpressionOp,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub rhs: Expression,
}

#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
pub struct ComparisonExpression {
	pub lhs: Expression,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub op:  ComparisonExpressionOp,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub rhs: Expression,
}

#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:LazyBooleanExpression")]
pub struct LazyBooleanExpression {
	pub lhs: Expression,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub op:  LazyBooleanExpressionOp,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub rhs: Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LazyBooleanExpressionOp {
	Or(token::OrOr),
	And(token::AndAnd),
}

/// `TypeCastExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "left")]
pub struct TypeCastExpression {
	pub lhs: Expression,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub as_: token::As,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:  TypeNoBounds,
}

/// `AssignmentExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:AssignmentExpression")]
pub struct AssignmentExpression {
	pub lhs: Expression,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub eq:  token::Eq,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub rhs: Expression,
}

/// `CompoundAssignmentExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = "skip:CompoundAssignmentExpression")]
pub struct CompoundAssignmentExpression {
	pub lhs: Expression,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub op:  CompoundAssignmentExpressionOp,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub rhs: Expression,
}

#[derive(PartialEq, Eq, Debug)]
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
