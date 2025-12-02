//! Extern crate

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{ident::Ident, token},
};

/// `ExternCrate`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "extern crate")]
pub struct ExternCrate {
	pub extern_:   token::Extern,
	pub crate_:    token::Crate,
	pub crate_ref: CrateRef,
	pub as_clause: Option<AsClause>,
	pub semi:      token::Semi,
}

/// `CrateRef`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum CrateRef {
	Self_(token::SelfLower),
	Ident(Ident),
}

/// `AsClause`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AsClause {
	pub as_:  token::As,
	pub name: AsClauseName,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AsClauseName {
	Underscore(token::Underscore),
	Ident(Ident),
}
