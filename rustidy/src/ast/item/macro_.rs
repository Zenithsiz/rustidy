//! Macro

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{
		attr::TokenTree,
		delimited::{Braced, Bracketed, Parenthesized},
		path::SimplePath,
		token,
	},
};

/// `MacroInvocationSemi`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroInvocationSemi {
	Parens(MacroInvocationSemiParens),
	Brackets(MacroInvocationSemiBrackets),
	Braces(MacroInvocationSemiBraces),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiParens {
	pub path:   SimplePath,
	pub not:    token::Not,
	pub tokens: Parenthesized<Vec<TokenTree>>,
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiBrackets {
	pub path:   SimplePath,
	pub not:    token::Not,
	pub tokens: Bracketed<Vec<TokenTree>>,
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiBraces {
	pub path:   SimplePath,
	pub not:    token::Not,
	pub tokens: Braced<Vec<TokenTree>>,
}
