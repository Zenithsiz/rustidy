//! Type path

// Imports
use {
	crate::expr::without_block::path::{GenericArgs, PathIdentSegment, TypePathFn},

	ast_util::{Punctuated, punct},
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `TypePath`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypePath {
	pub prefix:   Option<ast_token::PathSep>,
	#[format(prefix_ws(expr = Whitespace::REMOVE, if_ = self.prefix.is_some()))]
	#[format(args = punct::fmt(Whitespace::REMOVE, Whitespace::REMOVE))]
	pub segments: Punctuated<TypePathSegment, ast_token::PathSep>,
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
	pub sep:   Option<ast_token::PathSep>,
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
