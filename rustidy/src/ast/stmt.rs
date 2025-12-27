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
	let_: token::Let,
	#[parse(fatal)]
	pat:  PatternNoTopAlt,
	ty:   Option<LetStatementTy>,
	eq:   Option<LetStatementEq>,
	semi: token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementTy {
	colon: token::Colon,
	#[parse(fatal)]
	ty:    Type,
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
	eq:   token::Eq,
	#[parse(fatal)]
	expr: Expression,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementEqElse {
	eq:        token::Eq,
	// TODO: Except `LazyBooleanExpression` and ending with `}`.
	expr:      Expression,
	else_:     token::Else,
	#[parse(fatal)]
	else_expr: BlockExpression,
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
	expr: ExpressionWithoutBlock,
	semi: token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExpressionStatementWithBlock {
	expr: ExpressionWithBlock,
	semi: Option<token::Semi>,
}
