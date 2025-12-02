//! Visibility

// Imports
use {
	super::{delimited::Parenthesized, path::SimplePath, token},
	crate::{Format, Parse, Print},
};

/// `Visibility`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Visibility {
	pub pub_: token::Pub,
	pub path: Option<Parenthesized<VisibilityPath>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum VisibilityPath {
	Crate(token::Crate),
	Self_(token::SelfLower),
	Super(token::Super),
	In(VisibilityPathIn),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct VisibilityPathIn {
	pub in_:  token::In,
	#[parse(fatal)]
	pub path: SimplePath,
}
