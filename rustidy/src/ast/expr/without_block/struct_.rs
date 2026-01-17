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
			punct::{self, Punctuated},
			token,
			with_attrs::{self, WithOuterAttributes},
		},
	},
};

/// `StructExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a struct expression")]
#[parse(skip_if_tag = "skip:StructExpression")]
pub struct StructExpression {
	pub path:  PathInExpression,
	#[format(and_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub inner: Braced<Option<StructExpressionInner>>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExpressionInner {
	Fields(StructExprFields),
	Base(StructBase),
}

/// `StructExprFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructExprFields {
	#[format(and_with = punct::format(Format::prefix_ws_set_cur_indent, Format::prefix_ws_remove))]
	pub fields: Punctuated<StructExprField, token::Comma>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub end:    Option<StructExprFieldsEnd>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExprFieldsEnd {
	Base(StructExprFieldsEndBase),
	TrailingComma(token::Comma),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructExprFieldsEndBase {
	pub comma: token::Comma,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub base:  StructBase,
}

/// `StructExprField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructExprField(
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<StructExprFieldInner>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExprFieldInner {
	WithExpr(StructExprFieldInnerWithExpr),
	Ident(Identifier),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructExprFieldInnerWithExpr {
	pub start: StructExprFieldInnerWithExprStart,
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon: token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr:  Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructExprFieldInnerWithExprStart {
	Ident(Identifier),
	Tuple(TupleIndex),
}

/// `StructBase`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructBase {
	pub dot_dot: token::DotDot,
	#[format(and_with = Format::prefix_ws_remove)]
	pub expr:    Expression,
}
