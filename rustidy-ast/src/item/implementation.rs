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
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
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
	#[format(prefix_ws = Whitespace::remove)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ty:       Type,
	#[format(prefix_ws = Whitespace::set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub body:     BracedWithInnerAttributes<ImplBody>,
}

/// `TraitImpl`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitImpl {
	pub unsafe_:  Option<token::Unsafe>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.unsafe_.is_some()))]
	pub impl_:    token::Impl,
	#[format(prefix_ws = Whitespace::remove)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub const_:   Option<token::Const>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub not:      Option<token::Not>,
	#[format(prefix_ws = match self.not.is_some() {
		true => Whitespace::remove,
		false => Whitespace::set_single,
	})]
	pub trait_:   TypePath,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub for_:     token::For,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ty:       Type,
	#[format(prefix_ws = Whitespace::set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub body:     BracedWithInnerAttributes<ImplBody>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ImplBody(
	#[format(args = rustidy_format::vec::Args::from_prefix_ws(Whitespace::set_cur_indent))] pub Vec<AssociatedItem>,
);
