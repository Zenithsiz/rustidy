//! Macro

// Imports
use {
	crate::{
		attr::TokenTree,
		path::SimplePath,
		token,
		util::{Braced, Bracketed, Parenthesized},
	},
	rustidy_ast_util::delimited,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `MacroInvocationSemi`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroInvocationSemi {
	Parens(MacroInvocationSemiParens),
	Brackets(MacroInvocationSemiBrackets),
	Braces(MacroInvocationSemiBraces),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroInvocationSemiParens {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::fmt_preserve())]
	pub tokens: Parenthesized<MacroInvocationSemiTokens>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroInvocationSemiBrackets {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::fmt_preserve())]
	pub tokens: Bracketed<MacroInvocationSemiTokens>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroInvocationSemiBraces {
	pub path:   SimplePath,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:    token::Not,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(args = delimited::fmt_preserve())]
	pub tokens: Braced<MacroInvocationSemiTokens>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroInvocationSemiTokens(#[format(args = rustidy_format::vec::args_prefix_ws(Whitespace::PRESERVE))]
Vec<TokenTree>,);
