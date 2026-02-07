//! Implementation statement

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		trait_::AssociatedItem,
	},
	crate::{
		attr::BracedWithInnerAttributes,
		token,
		ty::{Type, TypePath},
	},
	rustidy_format::Format,
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
	#[format(before_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ty:       Type,
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub body:     BracedWithInnerAttributes<ImplBody>,
}

/// `TraitImpl`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitImpl {
	pub unsafe_:  Option<token::Unsafe>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.unsafe_.is_some()))]
	pub impl_:    token::Impl,
	#[format(before_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub const_:   Option<token::Const>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub not:      Option<token::Not>,
	#[format(and_with = match self.not.is_some() {
		true => Format::prefix_ws_remove,
		false => Format::prefix_ws_set_single,
	})]
	pub trait_:   TypePath,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub for_:     token::For,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ty:       Type,
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub body:     BracedWithInnerAttributes<ImplBody>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplBody(
	#[format(and_with = rustidy_format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))]
	pub  Vec<AssociatedItem>,
);
