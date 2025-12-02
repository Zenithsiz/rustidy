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
			attr::InnerAttrOrDocComment,
			delimited::Braced,
			token,
			ty::{Type, TypePath},
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
	impl_:    token::Impl,
	generics: Option<GenericParams>,
	ty:       Type,
	where_:   Option<WhereClause>,
	body:     Braced<ImplBody>,
}

/// `TraitImpl`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitImpl {
	unsafe_:  Option<token::Unsafe>,
	impl_:    token::Impl,
	generics: Option<GenericParams>,
	not:      Option<token::Not>,
	trait_:   TypePath,
	#[parse(fatal)]
	for_:     token::For,
	ty:       Type,
	where_:   Option<WhereClause>,
	body:     Braced<ImplBody>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplBody {
	attrs: Vec<InnerAttrOrDocComment>,
	items: Vec<AssociatedItem>,
}
