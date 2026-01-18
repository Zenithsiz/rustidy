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
use {
	crate::{
		Format,
		Parse,
		ParseRecursive,
		Print,
		parser::{FromRecursiveRoot, RecursiveWrapper},
	},
	rustidy_util::{Arena, ArenaData, ArenaIdx},
};

/// `Expression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[expect(clippy::use_self, reason = "`Parse` derive macro doesn't support `Self`")]
pub struct Expression(ArenaIdx<Expression>);

impl ArenaData for Expression {
	type Data = ExpressionInner;

	const ARENA: &'static Arena<Self> = &EXPRESSION_ARENA;
}

static EXPRESSION_ARENA: Arena<Expression> = Arena::new();

impl FromRecursiveRoot<ExpressionInner> for Expression {
	fn from_recursive_root(expr: ExpressionInner, _parser: &mut crate::Parser) -> Self {
		let idx = EXPRESSION_ARENA.push(expr);
		Self(idx)
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, ParseRecursive, Format, Print)]
#[parse(from = RecursiveWrapper::<ExpressionInner, ExpressionInner>)]
#[parse_recursive(root = ExpressionInner)]
pub enum ExpressionInner {
	#[parse_recursive(recursive)]
	WithoutBlock(ExpressionWithoutBlock),
	WithBlock(ExpressionWithBlock),
}
