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
		format,
	},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Implementation`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an implementation")]
pub enum Implementation {
	Inherent(InherentImpl),
	Trait(TraitImpl),
}

/// `InherentImpl`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InherentImpl {
	pub impl_:    token::Impl,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:       Type,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(and_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub body:     Braced<ImplBody>,
}

/// `TraitImpl`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitImpl {
	pub unsafe_:  Option<token::Unsafe>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.unsafe_.is_some()))]
	pub impl_:    token::Impl,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub const_:   Option<token::Const>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub not:      Option<token::Not>,
	#[format(and_with = match self.not.is_some() {
		true => Format::prefix_ws_remove,
		false => Format::prefix_ws_set_single,
	})]
	pub trait_:   TypePath,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub for_:     token::For,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:       Type,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(and_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub body:     Braced<ImplBody>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplBody(pub WithInnerAttributes<ImplBodyInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplBodyInner(
	#[format(and_with = format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))] pub Vec<AssociatedItem>,
);
