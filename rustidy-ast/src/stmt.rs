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
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Statement`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
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
#[derive(Parse, Format, Print)]
pub struct LetStatement(pub WithOuterAttributes<LetStatementInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a let statement")]
pub struct LetStatementInner {
	pub super_: Option<token::Super>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.super_.is_some()))]
	pub let_:   token::Let,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub pat:    PatternNoTopAlt,
	#[format(prefix_ws = Whitespace::remove)]
	pub ty:     Option<LetStatementTy>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub eq:     Option<LetStatementEq>,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementTy {
	pub colon: token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub ty:    Type,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LetStatementEq {
	Else(LetStatementEqElse),
	Normal(LetStatementEqNormal),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementEqNormal {
	pub eq:   token::Eq,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub expr: Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementEqElse {
	pub eq:        token::Eq,
	#[format(prefix_ws = Whitespace::set_single)]
	// TODO: Except `LazyBooleanExpression` and ending with `}`.
	pub expr: Expression,
	#[format(prefix_ws = Whitespace::set_single)]
	pub else_:     token::Else,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub else_expr: BlockExpression,
}

/// `ExpressionStatement`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ExpressionStatement {
	WithoutBlock(ExpressionStatementWithoutBlock),
	WithBlock(ExpressionStatementWithBlock),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExpressionStatementWithoutBlock {
	pub expr: ExpressionWithoutBlock,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi: token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExpressionStatementWithBlock {
	pub expr: ExpressionWithBlock,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi: Option<token::Semi>,
}
