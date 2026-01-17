//! Macro invocation

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{attr::DelimTokenTree, path::SimplePath, token},
};

/// `MacroInvocation`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a macro invocation")]
pub struct MacroInvocation {
	pub path: SimplePath,
	#[format(and_with = Format::prefix_ws_remove)]
	pub not:  token::Not,
	#[format(and_with = Format::prefix_ws_remove)]
	pub tree: DelimTokenTree,
}
