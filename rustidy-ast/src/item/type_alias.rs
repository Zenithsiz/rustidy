//! Type alias

// Imports
use {
	super::function::{GenericParams, TypeParamBounds, WhereClause},
	crate::{token, ty::Type},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `TypeAlias`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAlias {
	pub type_:    token::Type,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::remove)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::remove)]
	pub bounds:   Option<TypeAliasBounds>,
	#[format(prefix_ws = Whitespace::set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub eq:       Option<TypeAliasEq>,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi:     token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasBounds {
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::set_single)]
	pub bounds: TypeParamBounds,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasEq {
	pub eq:     token::Eq,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub ty:     Type,
	#[format(prefix_ws = Whitespace::set_cur_indent)]
	pub where_: Option<WhereClause>,
}
