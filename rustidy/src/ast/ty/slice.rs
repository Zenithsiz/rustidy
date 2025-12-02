//! Slice type

// Imports
use {
	super::Type,
	crate::{Format, Parse, Print, ast::delimited::Bracketed},
};

/// `SliceType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SliceType(Bracketed<Box<Type>>);
