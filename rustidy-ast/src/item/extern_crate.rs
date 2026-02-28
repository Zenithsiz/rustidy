//! Extern crate

// Imports
use {
	ast_literal::Identifier,
	ast_literal::token,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
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
