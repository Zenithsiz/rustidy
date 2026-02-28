//! Type path

// Imports
use {
	crate::expr::without_block::path::{GenericArgs, PathIdentSegment, TypePathFn},
	rustidy_ast_literal::token,
	rustidy_ast_util::{Punctuated, punct},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `TypePath`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypePath {
	pub prefix:   Option<token::PathSep>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.prefix.is_some()))]
	#[format(args = punct::fmt(Whitespace::REMOVE, Whitespace::REMOVE))]
	pub segments: Punctuated<TypePathSegment, token::PathSep>,
}

/// `TypePathSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypePathSegment {
	pub path:     PathIdentSegment,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<TypePathSegmentGenerics>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypePathSegmentGenerics {
	pub sep:   Option<token::PathSep>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.sep.is_some()))]
	pub inner: GenericArgsOrTypePathFn,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum GenericArgsOrTypePathFn {
	GenericArgs(GenericArgs),
	TypePathFn(TypePathFn),
}
