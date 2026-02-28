//! Union

// Imports
use {
	crate::util::Braced,
	super::{function::{GenericParams, WhereClause}, struct_::StructFields},
	ast_literal::Identifier,
	ast_util::delimited,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `Union`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Union {
	pub union:    ast_token::Union,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::INDENT)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(args = delimited::fmt_indent_if_non_blank())]
	pub fields:   Braced<Option<StructFields>>,
}
