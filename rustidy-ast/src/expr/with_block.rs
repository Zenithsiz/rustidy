//! Expressions with a block

// Modules
pub mod block;
pub mod match_;

// Exports
pub use self::{block::BlockExpression, match_::{MatchExpression, Scrutinee}};

// Imports
use {
	crate::{attr::{self, WithOuterAttributes}, pat::Pattern},
	super::Expression,
	ast_literal::LifetimeOrLabel,
	ast_util::{Longest, Punctuated, punct},
	format::{Format, Formattable, WhitespaceFormat},
	parse::{ParsableFrom, Parse, ParserTag},
	print::Print,
	util::Whitespace,
};

/// `ExpressionWithBlock`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExpressionWithBlock(
	// TODO: Should this ever be SINGLE?
	#[format(args = attr::with::fmt(Whitespace::INDENT))]
	pub WithOuterAttributes<ExpressionWithBlockInner>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ExpressionWithBlockInner {
	Block(BlockExpression),
	ConstBlock(ConstBlockExpression),
	UnsafeBlock(UnsafeBlockExpression),
	TryBlock(TryBlockExpression),
	Loop(LoopExpression),
	If(IfExpression),
	Match(MatchExpression),
}

/// `ConstBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ConstBlockExpression {
	pub const_: ast_token::Const,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:   BlockExpression,
}

/// `UnsafeBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct UnsafeBlockExpression {
	pub unsafe_: ast_token::Unsafe,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:    BlockExpression,
}

// Note: Nightly-only
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TryBlockExpression {
	pub try_: ast_token::Try,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: BlockExpression,
}

/// `IfExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "an if expression")]
pub struct IfExpression {
	pub if_:        ast_token::If,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub conditions: Conditions,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:       BlockExpression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub else_:      Option<IfExpressionElse>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct IfExpressionElse {
	pub else_: ast_token::Else,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub inner: IfExpressionElseInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum IfExpressionElseInner {
	Block(BlockExpression),
	If(Box<IfExpression>),
}

/// `Conditions`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(from = Longest::<LetChain, ConditionsExpr>)]
pub enum Conditions {
	Let(LetChain),
	Expr(ConditionsExpr)
}

impl ParsableFrom<Longest<LetChain, ConditionsExpr>> for Conditions {
	fn from_parsable(value: Longest<LetChain, ConditionsExpr>) -> Self {
		match value {
			Longest::Left(let_chain) => Self::Let(let_chain),
			Longest::Right(expr) => Self::Expr(expr),
		}
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ConditionsExpr(
	#[parse(with_tag = ParserTag::SkipStructExpression)]
	#[parse(with_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	Expression,
);

/// `LetChain`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetChain(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::SINGLE))]
	pub Punctuated<LetChainCondition, ast_token::AndAnd>,
);

/// `LetChainCondition`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum LetChainCondition {
	#[format(args = attr::with::fmt(Whitespace::SINGLE))]
	Let(WithOuterAttributes<LetChainConditionLet>),
	#[parse(with_tag = ParserTag::SkipStructExpression)]
	#[parse(with_tag = ParserTag::SkipLazyBooleanExpression)]
	#[parse(with_tag = ParserTag::SkipRangeExpr)]
	#[parse(with_tag = ParserTag::SkipRangeFromExpr)]
	#[parse(with_tag = ParserTag::SkipRangeInclusiveExpr)]
	#[parse(with_tag = ParserTag::SkipAssignmentExpression)]
	#[parse(with_tag = ParserTag::SkipCompoundAssignmentExpression)]
	#[parse(with_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	Expr(Expression),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetChainConditionLet {
	pub let_:      ast_token::Let,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat:       Pattern,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:        ast_token::Eq,
	#[parse(with_tag = ParserTag::SkipStructExpression)]
	#[parse(with_tag = ParserTag::SkipLazyBooleanExpression)]
	#[parse(with_tag = ParserTag::SkipRangeExpr)]
	#[parse(with_tag = ParserTag::SkipRangeFromExpr)]
	#[parse(with_tag = ParserTag::SkipRangeInclusiveExpr)]
	#[parse(with_tag = ParserTag::SkipAssignmentExpression)]
	#[parse(with_tag = ParserTag::SkipCompoundAssignmentExpression)]
	#[parse(with_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub scrutinee: Box<Scrutinee>,
}

/// `LoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LoopExpression {
	pub label: Option<LoopLabel>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.label.is_some()))]
	pub inner: LoopExpressionInner,
}

/// `LoopLabel`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LoopLabel {
	pub lifetime: LifetimeOrLabel,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:    ast_token::Colon,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum LoopExpressionInner {
	Infinite(InfiniteLoopExpression),
	Predicate(PredicateLoopExpression),
	Iterator(IteratorLoopExpression),
	LabelBlock(LabelBlockExpression),
}

/// `IteratorLoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct IteratorLoopExpression {
	pub for_: ast_token::For,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat:  Pattern,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub in_:  ast_token::In,
	#[parse(with_tag = ParserTag::SkipStructExpression)]
	#[parse(with_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub body: BlockExpression,
}

/// `PredicateLoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct PredicateLoopExpression {
	pub for_: ast_token::While,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub cond: Conditions,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub body: BlockExpression,
}

/// `InfiniteLoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct InfiniteLoopExpression {
	pub loop_: ast_token::Loop,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub body:  BlockExpression,
}

/// `LabelBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LabelBlockExpression(BlockExpression);
