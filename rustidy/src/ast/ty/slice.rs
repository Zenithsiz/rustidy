//! Slice type

// Imports
use {
	super::Type,
	crate::{Format, Parse, Print, ast::delimited::Bracketed},
};

/// `SliceType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SliceType(#[format(and_with = Bracketed::format_remove)] Bracketed<Box<Type>>);
