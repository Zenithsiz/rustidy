//! Statement

// Imports
use {
	super::{
		expr::{BlockExpression, Expression, ExpressionWithBlock, ExpressionWithoutBlock},
		item::Item,
		pat::PatternNoTopAlt,
		token,
		ty::Type,
		with_attrs::WithOuterAttributes,
	},
	crate::{Format, Parse, Print},
};

/// `Statement`
#[derive(PartialEq, Eq, Clone, Debug)]
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatement(pub WithOuterAttributes<LetStatementInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a let statement")]
pub struct LetStatementInner {
	pub super_: Option<token::Super>,
	pub let_:   token::Let,
	#[parse(fatal)]
	pub pat:    PatternNoTopAlt,
	pub ty:     Option<LetStatementTy>,
	pub eq:     Option<LetStatementEq>,
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementTy {
	pub colon: token::Colon,
	#[parse(fatal)]
	pub ty:    Type,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LetStatementEq {
	Else(LetStatementEqElse),
	Normal(LetStatementEqNormal),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementEqNormal {
	pub eq:   token::Eq,
	#[parse(fatal)]
	pub expr: Expression,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementEqElse {
	pub eq:        token::Eq,
	// TODO: Except `LazyBooleanExpression` and ending with `}`.
	pub expr:      Expression,
	pub else_:     token::Else,
	#[parse(fatal)]
	pub else_expr: BlockExpression,
}

/// `ExpressionStatement`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ExpressionStatement {
	WithoutBlock(ExpressionStatementWithoutBlock),
	WithBlock(ExpressionStatementWithBlock),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExpressionStatementWithoutBlock {
	pub expr: ExpressionWithoutBlock,
	pub semi: token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExpressionStatementWithBlock {
	pub expr: ExpressionWithBlock,
	pub semi: Option<token::Semi>,
}
