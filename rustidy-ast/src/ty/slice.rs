//! Slice type

// Imports
use {
	crate::util::Bracketed,
	super::Type,
	rustidy_ast_util::delimited,
	rustidy_format::{Format, Formattable},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `SliceType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct SliceType(#[format(args = delimited::FmtRemove)]
Bracketed<Box<Type>>);
