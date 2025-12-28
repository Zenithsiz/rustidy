//! Struct

// Imports
use {
	super::{literal::TupleIndex, path::PathInExpression},
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			delimited::Braced,
			expr::Expression,
			ident::Identifier,
			punct::Punctuated,
			token,
			with_attrs::WithOuterAttributes,
		},
	},
};

/// `StructExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a struct expression")]
#[parse(skip_if_tag = "skip:StructExpression")]
pub struct StructExpression {
	pub path:  PathInExpression,
	pub inner: Braced<Option<StructExpressionInner>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExpressionInner {
	Fields(StructExprFields),
	Base(StructBase),
}

/// `StructExprFields`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructExprFields {
	pub fields: Punctuated<StructExprField, token::Comma>,
	pub end:    Option<StructExprFieldsEnd>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExprFieldsEnd {
	Base(StructExprFieldsEndBase),
	TrailingComma(token::Comma),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructExprFieldsEndBase {
	pub comma: token::Comma,
	pub base:  StructBase,
}

/// `StructExprField`
type StructExprField = WithOuterAttributes<StructExprFieldInner>;

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExprFieldInner {
	WithExpr(StructExprFieldInnerWithExpr),
	Ident(Identifier),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructExprFieldInnerWithExpr {
	pub start: StructExprFieldInnerWithExprStart,
	pub colon: token::Colon,
	pub expr:  Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExprFieldInnerWithExprStart {
	Ident(Identifier),
	Tuple(TupleIndex),
}

/// `StructBase`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructBase {
	pub dot_dot: token::DotDot,
	pub expr:    Box<Expression>,
}
