//! Slice type

// Imports
use {
	super::Type,
	crate::{Format, Print, ast::delimited::Bracketed},
	rustidy_parse::Parse,
};

/// `SliceType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SliceType(#[format(and_with = Bracketed::format_remove)] Bracketed<Box<Type>>);
