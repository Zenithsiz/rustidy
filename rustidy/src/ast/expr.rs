//! Expression

// Modules
pub mod with_block;
pub mod without_block;

// Exports
pub use self::{
	with_block::{BlockExpression, ExpressionWithBlock, MatchExpression, Scrutinee},
	without_block::{
		ArrayExpression,
		AwaitExpression,
		CallExpression,
		ClosureExpression,
		ContinueExpression,
		ExpressionWithoutBlock,
		FieldExpression,
		GroupedExpression,
		IndexExpression,
		IntegerLiteral,
		LiteralExpression,
		MacroInvocation,
		MethodCallExpression,
		OperatorExpression,
		PathExpression,
		ReturnExpression,
		StringLiteral,
		StructExpression,
		TupleExpression,
		TupleIndexingExpression,
	},
};

// Imports
use crate::{Format, Parse, ParseRecursive, Print, parser::RecursiveWrapper};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, ParseRecursive, Format, Print)]
#[parse(from = RecursiveWrapper::<Expression, Expression>)]
#[parse_recursive(root = Expression)]
pub enum Expression {
	#[parse_recursive(recursive)]
	WithoutBlock(ExpressionWithoutBlock),
	WithBlock(ExpressionWithBlock),
}
