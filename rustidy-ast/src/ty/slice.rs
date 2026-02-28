//! Slice type

// Imports
use {
	crate::util::Bracketed,
	super::Type,
	ast_util::delimited,
	format::{Format, Formattable},
	parse::Parse,
	print::Print,
};

/// `SliceType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct SliceType(#[format(args = delimited::FmtRemove)] Bracketed<Box<Type>>);
