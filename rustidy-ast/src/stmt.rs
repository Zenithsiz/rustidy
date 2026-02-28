//! Statement

// Imports
use {
	crate::attr,
	super::{
		attr::WithOuterAttributes,
		expr::{BlockExpression, Expression, ExpressionWithBlock, ExpressionWithoutBlock},
		item::Item,
		pat::PatternNoTopAlt,
		ty::Type,
	},

	format::{Format, Formattable, WhitespaceFormat},
	parse::{Parse, ParseError, Parser, ParserError},
	print::Print,
	util::{ArenaData, ArenaIdx, Whitespace},
};

/// `Item`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Statement(pub ArenaIdx<StatementInner>);

/// `Statement`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(ArenaData)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a statement")]
pub enum StatementInner {
	Empty(ast_token::Semi),
	Let(LetStatement),
	Expression(ExpressionStatement),
	Item(Item),
}

/// `LetStatement`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetStatement(
	#[format(args = attr::with::fmt(Whitespace::INDENT))]
	pub WithOuterAttributes<LetStatementInner>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a let statement")]
pub struct LetStatementInner {
	pub super_: Option<ast_token::Super>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.super_.is_some()))]
	pub let_:   ast_token::Let,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat:    PatternNoTopAlt,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub ty:     Option<LetStatementTy>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:     Option<LetStatementEq>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   ast_token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetStatementTy {
	pub colon: ast_token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Type,
}

#[derive(PartialEq, Eq, Clone, Debug)]
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

		match parser.try_parse::<ast_token::Else>()? {
			Ok(else_) => {
				let else_expr = parser.parse()?;
				Ok(Self::Else(
					LetStatementEqElse { eq, expr, else_, else_expr, }
				))
			},
			Err(_) => Ok(Self::Normal(LetStatementEqNormal { eq, expr })),
		}
	}
}
#[derive(derive_more::Debug, derive_more::From, ParseError)]
pub enum LetStatementEqError {
	#[parse_error(transparent)]
	Eq(ParserError<ast_token::Eq>),

	#[parse_error(transparent)]
	Expr(ParserError<Expression>),

	#[parse_error(transparent)]
	Else(ParserError<ast_token::Else>),

	#[parse_error(transparent)]
	#[parse_error(fatal)]
	ElseExpr(ParserError<BlockExpression>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetStatementEqNormal {
	pub eq:   ast_token::Eq,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LetStatementEqElse {
	pub eq:        ast_token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	// TODO: Except `LazyBooleanExpression` and ending with `}`.
	pub expr:      Expression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub else_:     ast_token::Else,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub else_expr: BlockExpression,
}

/// `ExpressionStatement`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ExpressionStatement {
	WithoutBlock(ExpressionStatementWithoutBlock),
	WithBlock(ExpressionStatementWithBlock),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExpressionStatementWithoutBlock {
	pub expr: ExpressionWithoutBlock,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi: ast_token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ExpressionStatementWithBlock {
	pub expr: ExpressionWithBlock,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi: Option<ast_token::Semi>,
}
