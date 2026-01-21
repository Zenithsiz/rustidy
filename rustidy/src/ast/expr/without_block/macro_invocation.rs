//! Macro invocation

// Imports
use {
	crate::ast::{attr::DelimTokenTree, path::SimplePath, token},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `MacroInvocation`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a macro invocation")]
pub struct MacroInvocation {
	pub path: SimplePath,
	#[format(before_with = Format::prefix_ws_remove)]
	pub not:  token::Not,
	#[format(before_with = Format::prefix_ws_remove)]
	pub tree: DelimTokenTree,
}
