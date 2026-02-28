//! Macro invocation

// Imports
use {
	crate::{attr::DelimTokenTree, path::SimplePath},
	rustidy_ast_literal::token,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `MacroInvocation`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a macro invocation")]
pub struct MacroInvocation {
	pub path: SimplePath,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:  token::Not,
	#[format(prefix_ws = match self.tree {
		DelimTokenTree::Braces(_) => Whitespace::SINGLE,
		_ => Whitespace::REMOVE,
	})]
	pub tree: DelimTokenTree,
}
