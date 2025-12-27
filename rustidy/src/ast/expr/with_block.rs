//! Expressions with a block

// Modules
pub mod block;
pub mod match_;

// Exports
pub use self::{
	block::BlockExpression,
	match_::{MatchExpression, Scrutinee},
};

// Imports
use {
	super::Expression,
	crate::{
		Format,
		Parse,
		Print,
		ast::{lifetime::LifetimeOrLabel, pat::Pattern, punct::Punctuated, token, with_attrs::WithOuterAttributes},
	},
};

/// `ExpressionWithBlock`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExpressionWithBlock(pub WithOuterAttributes<ExpressionWithBlockInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
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
#[derive(Parse, Format, Print)]
pub struct ConstBlockExpression {
	const_: token::Const,
	#[parse(fatal)]
	expr:   BlockExpression,
}

/// `UnsafeBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UnsafeBlockExpression {
	unsafe_: token::Unsafe,
	#[parse(fatal)]
	expr:    BlockExpression,
}

// TODO: The specification doesn't have this, so we need to refine it
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TryBlockExpression {
	unsafe_: token::Try,
	#[parse(fatal)]
	expr:    BlockExpression,
}

/// `IfExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an if expression")]
pub struct IfExpression {
	if_:        token::If,
	#[parse(fatal)]
	conditions: Conditions,
	expr:       BlockExpression,
	else_:      Option<IfExpressionElse>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IfExpressionElse {
	else_: token::Else,
	#[parse(fatal)]
	inner: IfExpressionElseInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum IfExpressionElseInner {
	Block(BlockExpression),
	If(Box<IfExpression>),
}

/// `Conditions`
// TODO: The reference only mentions struct expression (and others for let chains),
//       but we also cannot parse anything that ends in a block
//       expression, so we block that too.
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum Conditions {
	LetChain(LetChain),
	#[parse(with_tag = "skip:StructExpression")]
	#[parse(with_tag = "skip:BlockExpression")]
	Expr(Box<Expression>),
}

/// `LetChain`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetChain(pub Punctuated<LetChainCondition, token::AndAnd>);

/// `LetChainCondition`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LetChainCondition {
	Let(WithOuterAttributes<LetChainConditionLet>),
	#[parse(with_tag = "skip:BlockExpression")]
	#[parse(with_tag = "skip:StructExpression")]
	#[parse(with_tag = "skip:LazyBooleanExpression")]
	#[parse(with_tag = "skip:RangeExpr")]
	#[parse(with_tag = "skip:RangeFromExpr")]
	#[parse(with_tag = "skip:RangeInclusiveExpr")]
	#[parse(with_tag = "skip:AssignmentExpression")]
	#[parse(with_tag = "skip:CompoundAssignmentExpression")]
	Expr(Box<Expression>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetChainConditionLet {
	let_:      token::Let,
	#[parse(fatal)]
	pat:       Pattern,
	eq:        token::Eq,
	#[parse(with_tag = "skip:BlockExpression")]
	#[parse(with_tag = "skip:StructExpression")]
	#[parse(with_tag = "skip:LazyBooleanExpression")]
	#[parse(with_tag = "skip:RangeExpr")]
	#[parse(with_tag = "skip:RangeFromExpr")]
	#[parse(with_tag = "skip:RangeInclusiveExpr")]
	#[parse(with_tag = "skip:AssignmentExpression")]
	#[parse(with_tag = "skip:CompoundAssignmentExpression")]
	scrutinee: Box<Scrutinee>,
}

/// `LoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LoopExpression {
	label: Option<LoopLabel>,
	inner: LoopExpressionInner,
}

/// `LoopLabel`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LoopLabel {
	lifetime: LifetimeOrLabel,
	colon:    token::Colon,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LoopExpressionInner {
	Infinite(InfiniteLoopExpression),
	Predicate(PredicateLoopExpression),
	Iterator(IteratorLoopExpression),
	LabelBlock(LabelBlockExpression),
}

/// `IteratorLoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IteratorLoopExpression {
	for_: token::For,
	pat:  Pattern,
	in_:  token::In,
	#[parse(with_tag = "skip:StructExpression")]
	expr: Box<Expression>,
	body: BlockExpression,
}

/// `PredicateLoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PredicateLoopExpression {
	for_: token::While,
	cond: Conditions,
	body: BlockExpression,
}

/// `InfiniteLoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InfiniteLoopExpression {
	loop_: token::Loop,
	body:  BlockExpression,
}

/// `LabelBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LabelBlockExpression(BlockExpression);
