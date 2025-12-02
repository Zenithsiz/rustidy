//! Union

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		struct_::StructFields,
	},
	crate::{
		Format,
		Parse,
		Print,
		ast::{delimited::Braced, ident::Ident, token},
	},
};

/// `Union`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Union {
	union:    token::Union,
	#[parse(fatal)]
	ident:    Ident,
	generics: Option<GenericParams>,
	where_:   Option<WhereClause>,
	fields:   Braced<Option<StructFields>>,
}
