//! Statement

// Imports
use {
	super::{
		attr::WithOuterAttributes,
		expr::{BlockExpression, Expression, ExpressionWithBlock, ExpressionWithoutBlock},
		item::Item,
		pat::PatternNoTopAlt,
		token,
		ty::Type,
	},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, ParseError, Parser, ParserError},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Statement`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a statement")]
pub enum Statement {
	Empty(token::Semi),
	Let(LetStatement),
	Expression(ExpressionStatement),
	Item(Item),
}

/// `LetStatement`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetStatement(pub WithOuterAttributes<LetStatementInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a let statement")]
pub struct LetStatementInner {
	pub super_: Option<token::Super>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.super_.is_some()))]
	pub let_:   token::Let,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat:    PatternNoTopAlt,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub ty:     Option<LetStatementTy>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:     Option<LetStatementEq>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetStatementTy {
	pub colon: token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Type,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Formattable, Format, Print)]
pub enum LetStatementEq {
	Else(LetStatementEqElse),
	Normal(LetStatementEqNormal),
}

impl Parse for LetStatementEq {
	type Error = LetStatementEqError;

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let eq = parser.parse()?;
		let expr = parser.parse()?;

		match parser.try_parse::<token::Else>()? {
			Ok(else_) => {
				let else_expr = parser.parse()?;
				Ok(Self::Else(LetStatementEqElse {
					eq,
					expr,
					else_,
					else_expr,
				}))
			},
			Err(_) => Ok(Self::Normal(LetStatementEqNormal { eq, expr })),
		}
	}
}
#[derive(derive_more::Debug, derive_more::From, ParseError)]
pub enum LetStatementEqError {
	#[parse_error(transparent)]
	Eq(ParserError<token::Eq>),

	#[parse_error(transparent)]
	Expr(ParserError<Expression>),

	#[parse_error(transparent)]
	Else(ParserError<token::Else>),

	#[parse_error(transparent)]
	#[parse_error(fatal)]
	ElseExpr(ParserError<BlockExpression>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetStatementEqNormal {
	pub eq:   token::Eq,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetStatementEqElse {
	pub eq:        token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	// TODO: Except `LazyBooleanExpression` and ending with `}`.
	pub expr: Expression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub else_:     token::Else,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub else_expr: BlockExpression,
}

/// `ExpressionStatement`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ExpressionStatement {
	WithoutBlock(ExpressionStatementWithoutBlock),
	WithBlock(ExpressionStatementWithBlock),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExpressionStatementWithoutBlock {
	pub expr: ExpressionWithoutBlock,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi: token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExpressionStatementWithBlock {
	pub expr: ExpressionWithBlock,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi: Option<token::Semi>,
}
