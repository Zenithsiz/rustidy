//! Underscore expression

// Imports
use {
	crate::{Format, Print, ast::token},
	rustidy_parse::Parse,
};

/// `UnderscoreExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UnderscoreExpression(token::Underscore);
