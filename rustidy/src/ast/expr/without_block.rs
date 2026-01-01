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
pub mod literal;
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
	literal::{IntegerLiteral, LiteralExpression, StringLiteral},
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
	super::Expression,
	crate::{
		Format,
		Parse,
		ParseRecursive,
		Print,
		ast::{token, with_attrs::WithOuterAttributes},
		parser::RecursiveWrapper,
	},
};

/// `ExpressionWithoutBlock`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, ParseRecursive, Format, Print)]
#[parse(from = RecursiveWrapper::<ExpressionWithoutBlock, Expression>)]
#[parse(skip_if_tag = "skip:ExpressionWithoutBlock")]
#[parse_recursive(root = Expression)]
#[parse_recursive(transparent)]
pub struct ExpressionWithoutBlock(pub WithOuterAttributes<ExpressionWithoutBlockInner, Self>);

impl From<ExpressionWithoutBlockInner> for ExpressionWithoutBlock {
	fn from(expr: ExpressionWithoutBlockInner) -> Self {
		Self(WithOuterAttributes::without_attributes(expr))
	}
}

impl TryFrom<Expression> for ExpressionWithoutBlock {
	type Error = ();

	fn try_from(expr: Expression) -> Result<Self, Self::Error> {
		match expr {
			Expression::WithoutBlock(inner) => Ok(inner),
			Expression::WithBlock(_) => Err(()),
		}
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = ExpressionWithoutBlock)]
pub enum ExpressionWithoutBlockInner {
	Underscore(UnderscoreExpression),

	#[parse_recursive(recursive)]
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
	Continue(ContinueExpression),
	Break(BreakExpression),
	AsyncBlock(AsyncBlockExpression),
}

// TODO: The specification doesn't have this, so we need to refine it
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "right")]
pub struct DoYeetExpression {
	pub do_:   token::Do,
	pub yeet_: token::Yeet,
	pub expr:  Box<Expression>,
}
