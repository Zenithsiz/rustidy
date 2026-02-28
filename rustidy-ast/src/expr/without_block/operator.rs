//! Operator expression

// Imports
use {
	crate::{expr::{Expression, ExpressionInner}, ty::TypeNoBounds},
	super::{ExpressionWithoutBlockInner, Parse},

	format::{Format, Formattable, WhitespaceFormat},
	parse::{ParseRecursive, ParserTag},
	print::Print,
	util::Whitespace,
};

/// `OperatorExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "left")]
pub struct TryPropagationExpression {
	pub expr:     Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub question: ast_token::Question,
}

/// `BorrowExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum BorrowExpressionKindRef {
	And(ast_token::And),
	AndAnd(ast_token::AndAnd),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum BorrowExpressionKindRest {
	Mut(ast_token::Mut),
	RawConst((ast_token::Raw, ast_token::Const)),
	RawMut((ast_token::Raw, ast_token::Mut)),
}

/// `DereferenceExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "right")]
pub struct DereferenceExpression {
	pub star: ast_token::Star,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub expr: Expression,
}

/// `NegationExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "right")]
pub struct NegationExpression {
	pub token: NegationExpressionToken,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub expr:  Expression,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum NegationExpressionToken {
	Minus(ast_token::Minus),
	Not(ast_token::Not),
}

/// `ArithmeticOrLogicalExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ArithmeticOrLogicalExpressionOp {
	Plus(ast_token::Plus),
	Minus(ast_token::Minus),
	Star(ast_token::Star),
	Div(ast_token::Slash),
	Percent(ast_token::Percent),
	And(ast_token::And),
	Or(ast_token::Or),
	Caret(ast_token::Caret),
	Shl(ast_token::Shl),
	Shr(ast_token::Shr),
}

/// `ComparisonExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ComparisonExpressionOp {
	EqEq(ast_token::EqEq),
	Ne(ast_token::Ne),
	Ge(ast_token::Ge),
	Le(ast_token::Le),
	Gt(ast_token::Gt),
	Lt(ast_token::Lt),
}

/// `LazyBooleanExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum LazyBooleanExpressionOp {
	Or(ast_token::OrOr),
	And(ast_token::AndAnd),
}

/// `TypeCastExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "left")]
pub struct TypeCastExpression {
	pub lhs: Expression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub as_: ast_token::As,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:  TypeNoBounds,
}

/// `AssignmentExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = OperatorExpression)]
#[parse_recursive(kind = "fully")]
#[parse_recursive(skip_if_tag = ParserTag::SkipAssignmentExpression)]
pub struct AssignmentExpression {
	pub lhs: Expression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:  ast_token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub rhs: Expression,
}

/// `CompoundAssignmentExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum CompoundAssignmentExpressionOp {
	Plus(ast_token::PlusEq),
	Minus(ast_token::MinusEq),
	Star(ast_token::StarEq),
	Div(ast_token::SlashEq),
	Percent(ast_token::PercentEq),
	And(ast_token::AndEq),
	Or(ast_token::OrEq),
	Caret(ast_token::CaretEq),
	Shl(ast_token::ShlEq),
	Shr(ast_token::ShrEq),
}
