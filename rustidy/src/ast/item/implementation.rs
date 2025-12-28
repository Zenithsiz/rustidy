//! Implementation statement

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		trait_::AssociatedItem,
	},
	crate::{
		Format,
		ast::{
			delimited::Braced,
			token,
			ty::{Type, TypePath},
			with_attrs::WithInnerAttributes,
		},
		parser::Parse,
		print::Print,
	},
};

/// `Implementation`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an implementation")]
pub enum Implementation {
	Inherent(InherentImpl),
	Trait(TraitImpl),
}

/// `InherentImpl`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InherentImpl {
	pub impl_:    token::Impl,
	pub generics: Option<GenericParams>,
	pub ty:       Type,
	pub where_:   Option<WhereClause>,
	pub body:     Braced<WithInnerAttributes<Vec<AssociatedItem>>>,
}

/// `TraitImpl`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitImpl {
	pub unsafe_:  Option<token::Unsafe>,
	pub impl_:    token::Impl,
	pub generics: Option<GenericParams>,
	pub not:      Option<token::Not>,
	pub trait_:   TypePath,
	#[parse(fatal)]
	pub for_:     token::For,
	pub ty:       Type,
	pub where_:   Option<WhereClause>,
	pub body:     Braced<WithInnerAttributes<Vec<AssociatedItem>>>,
}
