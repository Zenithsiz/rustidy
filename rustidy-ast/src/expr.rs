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
		MacroInvocation,
		MethodCallExpression,
		OperatorExpression,
		PathExpression,
		ReturnExpression,
		StructExpression,
		TupleExpression,
		TupleIndexingExpression,
	},
};

// Imports
use {
	rustidy_format::{Format, Formattable},
	rustidy_parse::{FromRecursiveRoot, Parse, ParseRecursive, Parser, RecursiveWrapper},
	rustidy_print::Print,
	rustidy_util::{ArenaData, ArenaIdx},
};

/// `Expression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Expression(pub ArenaIdx<ExpressionInner>);

impl FromRecursiveRoot<ExpressionInner> for Expression {
	fn from_recursive_root(expr: ExpressionInner, _parser: &mut Parser) -> Self {
		let idx = ArenaIdx::new(expr);
		Self(idx)
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, ParseRecursive, Formattable, Format, Print)]
#[parse(from = RecursiveWrapper::<ExpressionInner, ExpressionInner>)]
#[parse_recursive(root = ExpressionInner)]
pub enum ExpressionInner {
	#[parse_recursive(recursive)]
	WithoutBlock(ExpressionWithoutBlock),
	WithBlock(ExpressionWithBlock),
}
