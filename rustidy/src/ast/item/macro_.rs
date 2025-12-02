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
	path:   SimplePath,
	not:    token::Not,
	tokens: Parenthesized<Vec<TokenTree>>,
	semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiBrackets {
	path:   SimplePath,
	not:    token::Not,
	tokens: Bracketed<Vec<TokenTree>>,
	semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroInvocationSemiBraces {
	path:   SimplePath,
	not:    token::Not,
	tokens: Braced<Vec<TokenTree>>,
}
