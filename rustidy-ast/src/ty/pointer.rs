//! Tuple type

// Imports
use {
	crate::token,
	super::TypeNoBounds,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `RawPointerType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RawPointerType {
	pub star: token::Star,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub kind: RawPointerTypeKind,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:   Box<TypeNoBounds>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum RawPointerTypeKind {
	Const(token::Const),
	Mut(token::Mut),
}
