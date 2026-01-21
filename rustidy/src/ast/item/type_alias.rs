//! Type alias

// Imports
use {
	super::function::{GenericParams, TypeParamBounds, WhereClause},
	crate::ast::{token, ty::Type},
	rustidy_ast_util::Identifier,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `TypeAlias`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAlias {
	pub type_:    token::Type,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(before_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(before_with = Format::prefix_ws_remove)]
	pub bounds:   Option<TypeAliasBounds>,
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub eq:       Option<TypeAliasEq>,
	#[format(before_with = Format::prefix_ws_remove)]
	pub semi:     token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasBounds {
	pub colon:  token::Colon,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub bounds: TypeParamBounds,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasEq {
	pub eq:     token::Eq,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ty:     Type,
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
	pub where_: Option<WhereClause>,
}
