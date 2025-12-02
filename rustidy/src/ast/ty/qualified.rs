//! Qualified path in

// Imports
use {
	super::path::TypePathSegment,
	crate::{
		Format,
		Parse,
		Print,
		ast::{at_least::AtLeast1, expr::without_block::path::QualifiedPathType, token},
	},
};

/// `QualifiedPathInType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct QualifiedPathInType {
	qualified: QualifiedPathType,
	segments:  AtLeast1<(token::PathSep, TypePathSegment)>,
}
