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
	ast_literal::{IntegerLiteral, LiteralExpression, StringLiteral},
	format::{Format, Formattable},
	parse::{
		FromRecursiveRoot,
		Parse,
		ParseRecursive,
		Parser,
		ParserError,
		RecursiveWrapper,
	},
	print::Print,
	util::{ArenaData, ArenaIdx},
};

/// `Expression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Expression(pub ArenaIdx<ExpressionInner>);

impl Expression {
	/// Gets a literal out of this expression, if it is one.
	#[must_use]
	pub fn as_literal(&self) -> Option<&LiteralExpression> {
		self.0
			.try_as_without_block_ref()?.0
			.inner
			.try_as_literal_ref()
	}

	/// Gets a string literal out of this expression, if it is one.
	#[must_use]
	pub fn as_string_literal(&self) -> Option<&StringLiteral> {
		self.as_literal()?.try_as_string_ref()
	}

	/// Gets an integer literal out of this expression, if it is one.
	#[must_use]
	pub fn as_integer_literal(&self) -> Option<&IntegerLiteral> {
		self.as_literal()?.try_as_integer_ref()
	}
}

impl FromRecursiveRoot<ExpressionInner> for Expression {
	fn from_recursive_root(expr: ExpressionInner, _parser: &mut Parser) -> Self {
		let idx = ArenaIdx::new(expr);
		Self(idx)
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(derive_more::From, derive_more::TryInto)]
#[derive(strum::EnumTryAs)]
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

#[derive(derive_more::Debug, derive_more::From, parse::ParseError)]
pub enum ExpressionInnerError {
	#[parse_error(transparent)]
	Attributes(ParserError<Vec<OuterAttrOrDocComment>>),

	#[parse_error(transparent)]
	Inner(ParserError<RecursiveWrapper<ExpressionInner, ExpressionInner>>),
}
