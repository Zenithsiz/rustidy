//! Underscore expression

// Imports
use crate::{Format, Parse, Print, ast::token};

/// `UnderscoreExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UnderscoreExpression(token::Underscore);
