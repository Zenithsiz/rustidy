//! Macro

// Imports
use {
	crate::{
		attr::TokenTree,
		path::SimplePath,
		token,
		util::{Braced, Bracketed, Parenthesized},
	},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `MacroInvocationSemi`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroInvocationSemi {
	Parens(MacroInvocationSemiParens),
	Brackets(MacroInvocationSemiBrackets),
	Braces(MacroInvocationSemiBraces),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiParens {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::remove)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::remove)]
	pub tokens: Parenthesized<Vec<TokenTree>>,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiBrackets {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::remove)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::remove)]
	pub tokens: Bracketed<Vec<TokenTree>>,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiBraces {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::remove)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::set_single)]
	pub tokens: Braced<Vec<TokenTree>>,
}
