//! Underscore expression

// Imports
use {crate::token, rustidy_format::Format, rustidy_parse::Parse, rustidy_print::Print};

/// `UnderscoreExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UnderscoreExpression(token::Underscore);
