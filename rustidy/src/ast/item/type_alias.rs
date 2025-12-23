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
	type_:    token::Type,
	#[parse(fatal)]
	ident:    Identifier,
	generics: Option<GenericParams>,
	bounds:   Option<TypeAliasBounds>,
	where_:   Option<WhereClause>,
	eq:       Option<TypeAliasEq>,
	semi:     token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasBounds {
	colon:  token::Colon,
	bounds: TypeParamBounds,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeAliasEq {
	eq:     token::Eq,
	#[parse(fatal)]
	ty:     Type,
	where_: Option<WhereClause>,
}
