//! Union

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		struct_::StructFields,
	},
	crate::{token, util::Braced},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Union`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Union {
	pub union:    token::Union,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::remove)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub fields:   Braced<Option<StructFields>>,
}
