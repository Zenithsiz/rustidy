//! Struct

// Imports
use {
	super::{TupleIndex, path::PathInExpression},
	crate::{attr::WithOuterAttributes, expr::Expression, token, util::Braced},
	rustidy_ast_util::{Identifier, Punctuated, delimited, punct},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `StructExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a struct expression")]
#[parse(skip_if_tag = ParserTag::SkipStructExpression)]
pub struct StructExpression {
	pub path:  PathInExpression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(indent)]
	#[format(args = delimited::fmt_indent_if_non_blank())]
	pub inner: Braced<Option<StructExpressionInner>>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructExpressionInner {
	Fields(StructExprFields),
	Base(StructBase),
}

/// `StructExprFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructExprFields {
	#[format(args = punct::fmt(Whitespace::CUR_INDENT, Whitespace::REMOVE))]
	pub fields: Punctuated<StructExprField, token::Comma>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub end:    Option<StructExprFieldsEnd>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructExprFieldsEnd {
	Base(StructExprFieldsEndBase),
	TrailingComma(token::Comma),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructExprFieldsEndBase {
	pub comma: token::Comma,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub base:  StructBase,
}

/// `StructExprField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructExprField(pub WithOuterAttributes<StructExprFieldInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructExprFieldInner {
	WithExpr(StructExprFieldInnerWithExpr),
	Ident(Identifier),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructExprFieldInnerWithExpr {
	pub start: StructExprFieldInnerWithExprStart,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon: token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:  Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructExprFieldInnerWithExprStart {
	Ident(Identifier),
	Tuple(TupleIndex),
}

/// `StructBase`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructBase {
	pub dot_dot: token::DotDot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub expr:    Expression,
}
