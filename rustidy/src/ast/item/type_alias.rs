//! Type alias

// Imports
use {
	super::function::{GenericParams, TypeParamBounds, WhereClause},
	crate::{
		Format,
		Parse,
		Print,
		ast::{ident::Identifier, token, ty::Type},
	},
};

/// `TypeAlias`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAlias {
	pub type_:    token::Type,
	#[parse(fatal)]
	pub ident:    Identifier,
	pub generics: Option<GenericParams>,
	pub bounds:   Option<TypeAliasBounds>,
	pub where_:   Option<WhereClause>,
	pub eq:       Option<TypeAliasEq>,
	pub semi:     token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasBounds {
	pub colon:  token::Colon,
	pub bounds: TypeParamBounds,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasEq {
	pub eq:     token::Eq,
	#[parse(fatal)]
	pub ty:     Type,
	pub where_: Option<WhereClause>,
}
