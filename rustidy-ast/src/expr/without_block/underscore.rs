//! Underscore expression

// Imports
use {format::{Format, Formattable}, parse::Parse, print::Print};

/// `UnderscoreExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct UnderscoreExpression(ast_token::Underscore);
