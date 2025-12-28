//! Macro invocation

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{attr::DelimTokenTree, path::SimplePath, token},
};

/// `MacroInvocation`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a macro invocation")]
pub struct MacroInvocation {
	pub path: SimplePath,
	pub not:  token::Not,
	pub tree: DelimTokenTree,
}
