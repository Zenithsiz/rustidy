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
		ast::{delimited::Braced, ident::Identifier, token},
	},
};

/// `Union`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Union {
	pub union:    token::Union,
	#[parse(fatal)]
	pub ident:    Identifier,
	pub generics: Option<GenericParams>,
	pub where_:   Option<WhereClause>,
	pub fields:   Braced<Option<StructFields>>,
}
