//! Underscore expression

// Imports
use {
	crate::{Format, ast::token},
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `UnderscoreExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UnderscoreExpression(token::Underscore);
