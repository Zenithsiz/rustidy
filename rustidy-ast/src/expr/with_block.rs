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
		attr::{self, WithOuterAttributes},
		lifetime::LifetimeOrLabel,
		pat::Pattern,
		token,
	},
	rustidy_ast_util::{Longest, Punctuated, punct},
	rustidy_format::Format,
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
};

/// `ExpressionWithBlock`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExpressionWithBlock(
	#[format(and_with = attr::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<ExpressionWithBlockInner>,
);

#[derive(PartialEq, Eq, Debug)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstBlockExpression {
	pub const_: token::Const,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr:   BlockExpression,
}

/// `UnsafeBlockExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UnsafeBlockExpression {
	pub unsafe_: token::Unsafe,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr:    BlockExpression,
}

// Note: Nightly-only
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TryBlockExpression {
	pub try_: token::Try,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr: BlockExpression,
}

/// `IfExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an if expression")]
pub struct IfExpression {
	pub if_:        token::If,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub conditions: Conditions,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr:       BlockExpression,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub else_:      Option<IfExpressionElse>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IfExpressionElse {
	pub else_: token::Else,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub inner: IfExpressionElseInner,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum IfExpressionElseInner {
	Block(BlockExpression),
	If(Box<IfExpression>),
}

/// `Conditions`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Conditions(Longest<LetChain, ConditionsExpr>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
struct ConditionsExpr(
	#[parse(with_tag = ParserTag::SkipStructExpression)]
	#[parse(with_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	Expression,
);

/// `LetChain`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetChain(
	#[format(and_with = punct::format(Format::prefix_ws_set_single, Format::prefix_ws_set_single))]
	pub  Punctuated<LetChainCondition, token::AndAnd>,
);

/// `LetChainCondition`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LetChainCondition {
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

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetChainConditionLet {
	pub let_:      token::Let,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub pat:       Pattern,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub eq:        token::Eq,
	#[parse(with_tag = ParserTag::SkipStructExpression)]
	#[parse(with_tag = ParserTag::SkipLazyBooleanExpression)]
	#[parse(with_tag = ParserTag::SkipRangeExpr)]
	#[parse(with_tag = ParserTag::SkipRangeFromExpr)]
	#[parse(with_tag = ParserTag::SkipRangeInclusiveExpr)]
	#[parse(with_tag = ParserTag::SkipAssignmentExpression)]
	#[parse(with_tag = ParserTag::SkipCompoundAssignmentExpression)]
	#[parse(with_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub scrutinee: Box<Scrutinee>,
}

/// `LoopExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LoopExpression {
	pub label: Option<LoopLabel>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.label.is_some()))]
	pub inner: LoopExpressionInner,
}

/// `LoopLabel`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LoopLabel {
	pub lifetime: LifetimeOrLabel,
	#[format(before_with = Format::prefix_ws_remove)]
	pub colon:    token::Colon,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LoopExpressionInner {
	Infinite(InfiniteLoopExpression),
	Predicate(PredicateLoopExpression),
	Iterator(IteratorLoopExpression),
	LabelBlock(LabelBlockExpression),
}

/// `IteratorLoopExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IteratorLoopExpression {
	pub for_: token::For,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub pat:  Pattern,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub in_:  token::In,
	#[parse(with_tag = ParserTag::SkipStructExpression)]
	#[parse(with_tag = ParserTag::SkipOptionalTrailingBlockExpression)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub body: BlockExpression,
}

/// `PredicateLoopExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PredicateLoopExpression {
	pub for_: token::While,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub cond: Conditions,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub body: BlockExpression,
}

/// `InfiniteLoopExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InfiniteLoopExpression {
	pub loop_: token::Loop,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub body:  BlockExpression,
}

/// `LabelBlockExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LabelBlockExpression(BlockExpression);
