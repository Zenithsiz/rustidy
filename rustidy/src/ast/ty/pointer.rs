//! Tuple type

// Imports
use {
	super::TypeNoBounds,
	crate::{Format, Parse, Print, ast::token},
};

/// `RawPointerType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RawPointerType {
	pub star: token::Star,
	#[format(and_with = Format::prefix_ws_remove)]
	pub kind: RawPointerTypeKind,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:   Box<TypeNoBounds>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum RawPointerTypeKind {
	Const(token::Const),
	Mut(token::Mut),
}
