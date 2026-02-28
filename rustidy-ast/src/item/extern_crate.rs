//! Extern crate

// Imports
use {
	ast_literal::Identifier,

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
	pub extern_:   ast_token::Extern,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub crate_:    ast_token::Crate,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub crate_ref: CrateRef,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub as_clause: Option<AsClause>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:      ast_token::Semi,
}

/// `CrateRef`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum CrateRef {
	Self_(ast_token::SelfLower),
	Ident(Identifier),
}

/// `AsClause`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct AsClause {
	pub as_:  ast_token::As,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub name: AsClauseName,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum AsClauseName {
	Ident(Identifier),
	Underscore(ast_token::Underscore),
}
