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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Union {
	pub union:    token::Union,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(and_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub fields:   Braced<Option<StructFields>>,
}
