//! Integer literal

// Imports
use {
	super::SuffixNoE,
	crate::{Format, Parse, ParserStr, Print, ast::whitespace::Whitespace},
};


/// `INTEGER_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IntegerLiteral {
	#[format(whitespace)]
	pub ws:     Whitespace,
	pub inner:  IntegerLiteralInner,
	pub suffix: Option<SuffixNoE>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum IntegerLiteralInner {
	Decimal(DecLiteral),
	Binary(!),
	Octal(!),
	Hex(!),
}

/// `DEC_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an integer literal")]
#[parse(error(name = StartDigit, fmt = "Expected 0-9"))]
pub struct DecLiteral(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl DecLiteral {
	fn parse(s: &mut &str) -> Result<(), DecLiteralError> {
		if !s.starts_with(|ch: char| ch.is_ascii_digit()) {
			return Err(DecLiteralError::StartDigit);
		}
		*s = &s[1..];
		*s = s.trim_start_matches(|ch: char| ch.is_ascii_digit() || ch == '_');

		Ok(())
	}
}
