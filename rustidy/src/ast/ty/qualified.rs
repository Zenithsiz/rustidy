//! Qualified path in

// Imports
use {
	super::path::TypePathSegment,
	crate::{
		Format,
		Print,
		ast::{
			at_least::{self, AtLeast1},
			expr::without_block::path::QualifiedPathType,
			token,
		},
	},
	rustidy_parse::Parse,
};

/// `QualifiedPathInType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInType {
	pub qualified: QualifiedPathType,
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = at_least::format(Format::prefix_ws_remove))]
	pub segments:  AtLeast1<QualifiedPathInTypeSegment>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInTypeSegment {
	pub sep:     token::PathSep,
	#[format(and_with = Format::prefix_ws_remove)]
	pub segment: TypePathSegment,
}
