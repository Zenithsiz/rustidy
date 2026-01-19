//! Statement

// Imports
use {
	super::{
		expr::{BlockExpression, Expression, ExpressionWithBlock, ExpressionWithoutBlock},
		item::Item,
		pat::PatternNoTopAlt,
		token,
		ty::Type,
		with_attrs::{self, WithOuterAttributes},
	},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
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
pub struct LetStatement(
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<LetStatementInner>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a let statement")]
pub struct LetStatementInner {
	pub super_: Option<token::Super>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.super_.is_some()))]
	pub let_:   token::Let,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub pat:    PatternNoTopAlt,
	#[format(and_with = Format::prefix_ws_remove)]
	pub ty:     Option<LetStatementTy>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub eq:     Option<LetStatementEq>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementTy {
	pub colon: token::Colon,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LetStatementEqElse {
	pub eq:        token::Eq,
	#[format(and_with = Format::prefix_ws_set_single)]
	// TODO: Except `LazyBooleanExpression` and ending with `}`.
	pub expr: Expression,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub else_:     token::Else,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
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
	#[format(and_with = Format::prefix_ws_remove)]
	pub semi: token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExpressionStatementWithBlock {
	pub expr: ExpressionWithBlock,
	#[format(and_with = Format::prefix_ws_remove)]
	pub semi: Option<token::Semi>,
}
