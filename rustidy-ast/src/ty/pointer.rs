//! Tuple type

// Imports
use {
	super::TypeNoBounds,
	ast_literal::token,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
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
