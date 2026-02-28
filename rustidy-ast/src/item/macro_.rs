//! Macro

// Imports
use {
	crate::{
		attr::{DelimTokenTreeBraces, DelimTokenTreeBrackets, DelimTokenTreeParens},
		path::SimplePath,
	},
	ast_literal::token,
	format::{Format, Formattable, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `MacroInvocationSemi`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroInvocationSemi {
	Parens(MacroInvocationSemiParens),
	Brackets(MacroInvocationSemiBrackets),
	Braces(MacroInvocationSemiBraces),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroInvocationSemiParens {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub tokens: DelimTokenTreeParens,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroInvocationSemiBrackets {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub tokens: DelimTokenTreeBrackets,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroInvocationSemiBraces {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub tokens: DelimTokenTreeBraces,
}
