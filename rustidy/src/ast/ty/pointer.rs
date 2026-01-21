//! Tuple type

// Imports
use {super::TypeNoBounds, crate::ast::token, rustidy_format::Format, rustidy_parse::Parse, rustidy_print::Print};

/// `RawPointerType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RawPointerType {
	pub star: token::Star,
	#[format(before_with = Format::prefix_ws_remove)]
	pub kind: RawPointerTypeKind,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ty:   Box<TypeNoBounds>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum RawPointerTypeKind {
	Const(token::Const),
	Mut(token::Mut),
}
