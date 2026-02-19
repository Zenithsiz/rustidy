//! Union

// Imports
use {
	super::{function::{GenericParams, WhereClause}, struct_::StructFields},
	crate::{token, util::Braced},
	rustidy_ast_util::{Identifier, delimited},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Union`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Union {
	pub union:    token::Union,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::CUR_INDENT)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(indent)]
	#[format(args = delimited::fmt_indent_if_non_blank())]
	pub fields:   Braced<Option<StructFields>>,
}
