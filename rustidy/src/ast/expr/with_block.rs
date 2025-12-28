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
	pub const_: token::Const,
	#[parse(fatal)]
	pub expr:   BlockExpression,
}

/// `UnsafeBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UnsafeBlockExpression {
	pub unsafe_: token::Unsafe,
	#[parse(fatal)]
	pub expr:    BlockExpression,
}

// TODO: The specification doesn't have this, so we need to refine it
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TryBlockExpression {
	pub unsafe_: token::Try,
	#[parse(fatal)]
	pub expr:    BlockExpression,
}

/// `IfExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an if expression")]
pub struct IfExpression {
	pub if_:        token::If,
	#[parse(fatal)]
	pub conditions: Conditions,
	pub expr:       BlockExpression,
	pub else_:      Option<IfExpressionElse>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IfExpressionElse {
	pub else_: token::Else,
	#[parse(fatal)]
	pub inner: IfExpressionElseInner,
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
	pub let_:      token::Let,
	#[parse(fatal)]
	pub pat:       Pattern,
	pub eq:        token::Eq,
	#[parse(with_tag = "skip:BlockExpression")]
	#[parse(with_tag = "skip:StructExpression")]
	#[parse(with_tag = "skip:LazyBooleanExpression")]
	#[parse(with_tag = "skip:RangeExpr")]
	#[parse(with_tag = "skip:RangeFromExpr")]
	#[parse(with_tag = "skip:RangeInclusiveExpr")]
	#[parse(with_tag = "skip:AssignmentExpression")]
	#[parse(with_tag = "skip:CompoundAssignmentExpression")]
	pub scrutinee: Box<Scrutinee>,
}

/// `LoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LoopExpression {
	pub label: Option<LoopLabel>,
	pub inner: LoopExpressionInner,
}

/// `LoopLabel`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LoopLabel {
	pub lifetime: LifetimeOrLabel,
	pub colon:    token::Colon,
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
	pub for_: token::For,
	pub pat:  Pattern,
	pub in_:  token::In,
	#[parse(with_tag = "skip:StructExpression")]
	pub expr: Box<Expression>,
	pub body: BlockExpression,
}

/// `PredicateLoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PredicateLoopExpression {
	pub for_: token::While,
	pub cond: Conditions,
	pub body: BlockExpression,
}

/// `InfiniteLoopExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InfiniteLoopExpression {
	pub loop_: token::Loop,
	pub body:  BlockExpression,
}

/// `LabelBlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LabelBlockExpression(BlockExpression);
