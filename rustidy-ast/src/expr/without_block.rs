//! Expressions without a block

// Modules
pub mod array;
pub mod async_block;
pub mod await_;
pub mod break_;
pub mod call;
pub mod closure;
pub mod continue_;
pub mod field;
pub mod grouped;
pub mod index;
pub mod macro_invocation;
pub mod operator;
pub mod path;
pub mod range;
pub mod return_;
pub mod struct_;
pub mod tuple;
pub mod tuple_indexing;
pub mod underscore;

// Exports
pub use self::{
	array::ArrayExpression,
	async_block::AsyncBlockExpression,
	await_::AwaitExpression,
	break_::BreakExpression,
	call::{CallExpression, MethodCallExpression},
	closure::ClosureExpression,
	continue_::ContinueExpression,
	field::FieldExpression,
	grouped::GroupedExpression,
	index::IndexExpression,
	macro_invocation::MacroInvocation,
	operator::OperatorExpression,
	path::PathExpression,
	range::RangeExpression,
	return_::ReturnExpression,
	struct_::StructExpression,
	tuple::TupleExpression,
	tuple_indexing::TupleIndexingExpression,
	underscore::UnderscoreExpression,
};

// Imports
use {
	crate::{attr::{OuterAttrOrDocComment, WithOuterAttributes}, token},
	super::{Expression, ExpressionInner},
	rustidy_ast_literal::{IntegerLiteral, LiteralExpression},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, ParseRecursive, Parser, ParserError, ParserTag, RecursiveWrapper},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ExpressionWithoutBlock`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(transparent)]
#[parse_recursive(into_root = ExpressionInner)]
pub struct ExpressionWithoutBlock(pub WithOuterAttributes<ExpressionWithoutBlockInner>);

impl From<ExpressionWithoutBlockInner> for ExpressionWithoutBlock {
	fn from(expr: ExpressionWithoutBlockInner) -> Self {
		Self(WithOuterAttributes::without_attributes(expr))
	}
}

impl TryFrom<ExpressionWithoutBlock> for ExpressionWithoutBlockInner {
	type Error = ();

	fn try_from(expr: ExpressionWithoutBlock) -> Result<Self, Self::Error> {
		match expr.0.attrs.is_empty() {
			true => Ok(expr.0.inner),
			false => Err(()),
		}
	}
}

impl Parse for ExpressionWithoutBlock {
	type Error = ExpressionWithoutBlockError;

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		if parser
			.has_tag(ParserTag::SkipExpressionWithoutBlock) {
			return Err(ExpressionWithoutBlockError::Tag);
		}

		let attrs = parser.parse::<Vec<OuterAttrOrDocComment>>()?;
		let RecursiveWrapper(mut expr, _) = parser
			.parse::<RecursiveWrapper<Self, ExpressionInner>>()?;
		expr.0.attrs.extend(attrs);

		Ok(expr)
	}
}
#[derive(derive_more::Debug, derive_more::From, rustidy_parse::ParseError)]
pub enum ExpressionWithoutBlockError {
	#[parse_error(transparent)]
	Attributes(ParserError<Vec<OuterAttrOrDocComment>>),

	#[parse_error(transparent)]
	From(ParserError<RecursiveWrapper<ExpressionWithoutBlock, ExpressionInner>>),

	#[parse_error(fmt("Tag `{:?}` was present", ParserTag::SkipExpressionWithoutBlock))]
	#[debug("Tag({:?})", ParserTag::SkipExpressionWithoutBlock)]
	Tag,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(strum::EnumTryAs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlock)]
pub enum ExpressionWithoutBlockInner {
	DoYeet(DoYeetExpression),

	Literal(LiteralExpression),
	#[parse_recursive(recursive)]
	Operator(OperatorExpression),
	Grouped(GroupedExpression),
	#[parse_recursive(recursive)]
	Index(IndexExpression),
	#[parse_recursive(recursive)]
	Range(RangeExpression),

	MacroInvocation(MacroInvocation),

	#[parse_recursive(recursive)]
	MethodCall(MethodCallExpression),
	#[parse_recursive(recursive)]
	Call(CallExpression),
	#[parse_recursive(recursive)]
	Field(FieldExpression),
	#[parse_recursive(recursive)]
	TupleIndexing(TupleIndexingExpression),
	#[parse_recursive(recursive)]
	Await(AwaitExpression),
	Tuple(TupleExpression),
	Return(ReturnExpression),
	#[parse_recursive(recursive)]
	Closure(ClosureExpression),
	Struct(StructExpression),
	Array(ArrayExpression),
	Path(PathExpression),
	Underscore(UnderscoreExpression),
	Continue(ContinueExpression),
	Break(BreakExpression),
	AsyncBlock(AsyncBlockExpression),
}

// Note: Nightly-only
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct DoYeetExpression {
	pub do_:   token::Do,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub yeet_: token::Yeet,
	// TODO: This should be recursive
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:  Option<Expression>,
}

/// `TUPLE_INDEX`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleIndex(pub IntegerLiteral);
