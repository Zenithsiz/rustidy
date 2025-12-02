//! Struct

// Imports
use {
	super::{literal::TupleIndex, path::PathInExpression},
	crate::{
		Format,
		Parse,
		Print,
		ast::{expr::Expression, ident::Ident, punct::Punctuated, token, with_attrs::WithOuterAttributes},
	},
};

/// `StructExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a struct expression")]
#[parse(skip_if_tag = "skip:StructExpression")]
pub struct StructExpression {
	path:  PathInExpression,
	open:  token::BracesOpen,
	#[parse(fatal)]
	inner: Option<StructExpressionInner>,
	close: token::BracesClose,
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
	fields: Punctuated<StructExprField, token::Comma>,
	end:    Option<StructExprFieldsEnd>,
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
	comma: token::Comma,
	base:  StructBase,
}

/// `StructExprField`
type StructExprField = WithOuterAttributes<StructExprFieldInner>;

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExprFieldInner {
	WithExpr(StructExprFieldInnerWithExpr),
	Ident(Ident),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructExprFieldInnerWithExpr {
	start: StructExprFieldInnerWithExprStart,
	colon: token::Colon,
	expr:  Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExprFieldInnerWithExprStart {
	Ident(Ident),
	Tuple(TupleIndex),
}

/// `StructBase`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructBase {
	dot_dot: token::DotDot,
	expr:    Box<Expression>,
}
