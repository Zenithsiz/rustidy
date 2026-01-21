//! Macro

// Imports
use {
	crate::ast::{
		attr::TokenTree,
		path::SimplePath,
		token,
		util::{Braced, Bracketed, Parenthesized},
	},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
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
	#[format(before_with = Format::prefix_ws_remove)]
	pub not:    token::Not,
	#[format(before_with = Format::prefix_ws_remove)]
	pub tokens: Parenthesized<Vec<TokenTree>>,
	#[format(before_with = Format::prefix_ws_remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiBrackets {
	pub path:   SimplePath,
	#[format(before_with = Format::prefix_ws_remove)]
	pub not:    token::Not,
	#[format(before_with = Format::prefix_ws_remove)]
	pub tokens: Bracketed<Vec<TokenTree>>,
	#[format(before_with = Format::prefix_ws_remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiBraces {
	pub path:   SimplePath,
	#[format(before_with = Format::prefix_ws_remove)]
	pub not:    token::Not,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub tokens: Braced<Vec<TokenTree>>,
}
