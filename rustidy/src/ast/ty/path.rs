//! Type path

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{
		expr::without_block::path::{GenericArgs, PathIdentSegment, TypePathFn},
		punct::Punctuated,
		token,
	},
};

/// `TypePath`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePath {
	pub prefix:   Option<token::PathSep>,
	pub segments: Punctuated<TypePathSegment, token::PathSep>,
}

/// `TypePathSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathSegment {
	pub path:     PathIdentSegment,
	pub generics: Option<TypePathSegmentGenerics>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathSegmentGenerics {
	pub sep:   Option<token::PathSep>,
	pub inner: GenericArgsOrTypePathFn,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum GenericArgsOrTypePathFn {
	GenericArgs(GenericArgs),
	TypePathFn(TypePathFn),
}
