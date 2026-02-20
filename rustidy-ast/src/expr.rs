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
	crate::attr::OuterAttrOrDocComment,
	rustidy_format::{Format, Formattable},
	rustidy_parse::{
		FromRecursiveRoot,
		Parse,
		ParseRecursive,
		Parser,
		ParserError,
		RecursiveWrapper,
	},
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
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
pub enum ExpressionInner {
	#[parse_recursive(recursive)]
	WithoutBlock(ExpressionWithoutBlock),
	WithBlock(ExpressionWithBlock),
}

// TODO: Once we implement precedence and bubble all the attributes
//       to the top automatically, derive this impl
impl Parse for ExpressionInner {
	type Error = ExpressionInnerError;

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let attrs = parser.parse::<Vec<OuterAttrOrDocComment>>()?;
		let RecursiveWrapper(mut expr, _) = parser.parse::<RecursiveWrapper<Self, Self>>()?;

		match &mut expr {
			Self::WithoutBlock(expr) => expr.0.attrs.extend(attrs),
			Self::WithBlock(expr) => expr.0.attrs.extend(attrs),
		}

		Ok(expr)
	}
}

#[derive(derive_more::Debug, derive_more::From, rustidy_parse::ParseError)]
pub enum ExpressionInnerError {
	#[parse_error(transparent)]
	Attributes(ParserError<Vec<OuterAttrOrDocComment>>),

	#[parse_error(transparent)]
	Inner(ParserError<RecursiveWrapper<ExpressionInner, ExpressionInner>>),
}
