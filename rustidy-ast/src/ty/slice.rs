//! Slice type

// Imports
use {
	super::Type,
	crate::util::Bracketed,
	rustidy_ast_util::delimited,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `SliceType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SliceType(#[format(args = delimited::FmtArgs::remove((), (), ()))] Bracketed<Box<Type>>);
