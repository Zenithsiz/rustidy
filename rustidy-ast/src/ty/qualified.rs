//! Qualified path in

// Imports
use {
	super::path::TypePathSegment,
	crate::{expr::without_block::path::QualifiedPathType, token},
	rustidy_ast_util::{AtLeast1, at_least},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `QualifiedPathInType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInType {
	pub qualified: QualifiedPathType,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = at_least::args_prefix_ws(Whitespace::REMOVE))]
	pub segments:  AtLeast1<QualifiedPathInTypeSegment>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInTypeSegment {
	pub sep:     token::PathSep,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub segment: TypePathSegment,
}
