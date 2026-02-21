//! Struct

// Imports
use {
	super::{TupleIndex, path::PathInExpression},
	crate::{attr::WithOuterAttributes, expr::Expression, token, util::Braced},
	rustidy_ast_util::{Identifier, Punctuated, delimited, punct},
	rustidy_format::{Format, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `StructExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a struct expression")]
#[parse(skip_if_tag = ParserTag::SkipStructExpression)]
pub struct StructExpression {
	pub path:  PathInExpression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(
		args = delimited::fmt_single_or_indent_if_non_blank(50, StructExprFieldsFmt { field_prefix_ws: Whitespace::SINGLE }, StructExprFieldsFmt { field_prefix_ws: Whitespace::INDENT })
	)]
	pub inner: Braced<Option<StructExpressionInner>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "StructExprFieldsFmt"))]
pub enum StructExpressionInner {
	#[format(args = args)]
	Fields(StructExprFields),
	Base(StructBase),
}

/// `StructExprFields`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "StructExprFieldsFmt"))]
pub struct StructExprFields {
	#[format(args = punct::fmt(args.field_prefix_ws, Whitespace::REMOVE))]
	pub fields: Punctuated<StructExprField, token::Comma>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub end:    Option<StructExprFieldsEnd>,
}

#[derive(Clone, Copy, Debug)]
pub struct StructExprFieldsFmt {
	field_prefix_ws: WhitespaceConfig,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructExprFieldsEnd {
	Base(StructExprFieldsEndBase),
	TrailingComma(token::Comma),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructExprFieldsEndBase {
	pub comma: token::Comma,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub base:  StructBase,
}

/// `StructExprField`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructExprField(pub WithOuterAttributes<StructExprFieldInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructExprFieldInner {
	WithExpr(StructExprFieldInnerWithExpr),
	Ident(Identifier),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructExprFieldInnerWithExpr {
	pub start: StructExprFieldInnerWithExprStart,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon: token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:  Expression,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructExprFieldInnerWithExprStart {
	Ident(Identifier),
	Tuple(TupleIndex),
}

/// `StructBase`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructBase {
	pub dot_dot: token::DotDot,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub expr:    Expression,
}
