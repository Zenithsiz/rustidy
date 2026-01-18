//! Type alias

// Imports
use {
	super::function::{GenericParams, TypeParamBounds, WhereClause},
	crate::{
		Format,
		ast::{ident::Identifier, token, ty::Type},
	},
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub bounds:   Option<TypeAliasBounds>,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub eq:       Option<TypeAliasEq>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub semi:     token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasBounds {
	pub colon:  token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bounds: TypeParamBounds,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasEq {
	pub eq:     token::Eq,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:     Type,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_: Option<WhereClause>,
}
