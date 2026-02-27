//! Extern crate

// Imports
use {
	crate::token,
	rustidy_ast_literal::Identifier,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ExternCrate`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "extern crate")]
pub struct ExternCrate {
	pub extern_:   token::Extern,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub crate_:    token::Crate,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub crate_ref: CrateRef,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub as_clause: Option<AsClause>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:      token::Semi,
}

/// `CrateRef`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum CrateRef {
	Self_(token::SelfLower),
	Ident(Identifier),
}

/// `AsClause`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct AsClause {
	pub as_:  token::As,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub name: AsClauseName,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum AsClauseName {
	Ident(Identifier),
	Underscore(token::Underscore),
}
