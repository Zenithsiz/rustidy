//! Operator expression

// Imports
use {
	super::{ExpressionWithoutBlockInner, Parse},
	crate::{
		expr::{Expression, ExpressionInner},
		token,
		ty::TypeNoBounds,
	},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{ParseRecursive, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
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
	#[format(prefix_ws = Whitespace::REMOVE)]
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
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub rest: Option<BorrowExpressionKindRest>,
	#[format(prefix_ws = match self.rest.is_some() {
		true => Whitespace::SINGLE,
		false => Whitespace::REMOVE,
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
	#[format(prefix_ws = Whitespace::REMOVE)]
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
	#[format(prefix_ws = Whitespace::REMOVE)]
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
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub op:  ArithmeticOrLogicalExpressionOp,
	#[format(prefix_ws = Whitespace::SINGLE)]
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
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub op:  ComparisonExpressionOp,
	#[format(prefix_ws = Whitespace::SINGLE)]
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
#[parse_recursive(skip_if_tag = ParserTag::SkipLazyBooleanExpression)]
pub struct LazyBooleanExpression {
	pub lhs: Expression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub op:  LazyBooleanExpressionOp,
	#[format(prefix_ws = Whitespace::SINGLE)]
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
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub as_: token::As,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:  TypeNoBounds,
}

/// `AssignmentExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = ParserTag::SkipAssignmentExpression)]
pub struct AssignmentExpression {
	pub lhs: Expression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:  token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub rhs: Expression,
}

/// `CompoundAssignmentExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = ParserTag::SkipCompoundAssignmentExpression)]
pub struct CompoundAssignmentExpression {
	pub lhs: Expression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub op:  CompoundAssignmentExpressionOp,
	#[format(prefix_ws = Whitespace::SINGLE)]
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
