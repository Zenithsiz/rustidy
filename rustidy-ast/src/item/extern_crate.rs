//! Extern crate

// Imports
use {crate::token, rustidy_ast_util::Identifier, rustidy_format::Format, rustidy_parse::Parse, rustidy_print::Print};

/// `ExternCrate`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "extern crate")]
pub struct ExternCrate {
	pub extern_:   token::Extern,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub crate_:    token::Crate,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub crate_ref: CrateRef,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub as_clause: Option<AsClause>,
	#[format(before_with = Format::prefix_ws_remove)]
	pub semi:      token::Semi,
}

/// `CrateRef`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum CrateRef {
	Self_(token::SelfLower),
	Ident(Identifier),
}

/// `AsClause`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AsClause {
	pub as_:  token::As,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub name: AsClauseName,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AsClauseName {
	Underscore(token::Underscore),
	Ident(Identifier),
}
