//! Type path

// Imports
use {
	crate::ast::{
		expr::without_block::path::{GenericArgs, PathIdentSegment, TypePathFn},
		token,
	},
	rustidy_ast_util::{Punctuated, punct},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `TypePath`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePath {
	pub prefix:   Option<token::PathSep>,
	#[format(before_with(expr = Format::prefix_ws_remove, if = self.prefix.is_some()))]
	#[format(and_with = punct::format(Format::prefix_ws_remove, Format::prefix_ws_remove))]
	pub segments: Punctuated<TypePathSegment, token::PathSep>,
}

/// `TypePathSegment`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathSegment {
	pub path:     PathIdentSegment,
	#[format(before_with = Format::prefix_ws_remove)]
	pub generics: Option<TypePathSegmentGenerics>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathSegmentGenerics {
	pub sep:   Option<token::PathSep>,
	#[format(before_with(expr = Format::prefix_ws_remove, if = self.sep.is_some()))]
	pub inner: GenericArgsOrTypePathFn,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum GenericArgsOrTypePathFn {
	GenericArgs(GenericArgs),
	TypePathFn(TypePathFn),
}
