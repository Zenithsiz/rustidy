//! Union

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		struct_::StructFields,
	},
	crate::ast::{token, util::Braced},
	rustidy_ast_util::Identifier,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Union`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Union {
	pub union:    token::Union,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(before_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(before_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub fields:   Braced<Option<StructFields>>,
}
