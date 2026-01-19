//! Slice type

// Imports
use {
	super::Type,
	crate::ast::delimited::Bracketed,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `SliceType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SliceType(#[format(and_with = Bracketed::format_remove)] Bracketed<Box<Type>>);
