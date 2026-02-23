//! Qualified path in

// Imports
use {
	crate::{expr::without_block::path::QualifiedPathType, token},
	super::path::TypePathSegment,
	rustidy_ast_util::{AtLeast1, at_least},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `QualifiedPathInType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct QualifiedPathInType {
	pub qualified: QualifiedPathType,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = at_least::fmt_prefix_ws(Whitespace::REMOVE))]
	pub segments:  AtLeast1<QualifiedPathInTypeSegment>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct QualifiedPathInTypeSegment {
	pub sep:     token::PathSep,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub segment: TypePathSegment,
}
