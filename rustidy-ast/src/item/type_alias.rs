//! Type alias

// Imports
use {
	crate::ty::Type,
	super::function::{GenericParams, TypeParamBounds, WhereClause},
	ast_literal::{Identifier, token},
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `TypeAlias`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypeAlias {
	pub type_:    token::Type,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub bounds:   Option<TypeAliasBounds>,
	#[format(prefix_ws = Whitespace::INDENT)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:       Option<TypeAliasEq>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:     token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypeAliasBounds {
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bounds: TypeParamBounds,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TypeAliasEq {
	pub eq:     token::Eq,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:     Type,
	#[format(prefix_ws = Whitespace::INDENT)]
	pub where_: Option<WhereClause>,
}
