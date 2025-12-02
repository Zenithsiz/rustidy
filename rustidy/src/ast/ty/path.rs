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
	prefix:   Option<token::PathSep>,
	segments: Punctuated<TypePathSegment, token::PathSep>,
}

/// `TypePathSegment`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathSegment {
	path:     PathIdentSegment,
	generics: Option<TypePathSegmentGenerics>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypePathSegmentGenerics {
	sep:   Option<token::PathSep>,
	inner: GenericArgsOrTypePathFn,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum GenericArgsOrTypePathFn {
	GenericArgs(GenericArgs),
	TypePathFn(TypePathFn),
}
