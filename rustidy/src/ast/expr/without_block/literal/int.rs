//! Integer literal

// Imports
use {
	super::SuffixNoE,
	crate::{Format, Parse, ParseError, Parser, ParserStr, Print, ast::whitespace::Whitespace},
	std::fmt,
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
#[derive(Format, Print)]
pub struct DecLiteral(#[format(str)] pub ParserStr);

#[derive(Debug, ParseError)]
pub enum DecLiteralError {
	#[parse_error(fmt = "Expected 0-9")]
	StartDigit,
}

impl Parse for DecLiteral {
	type Error = DecLiteralError;

	fn name() -> Option<impl fmt::Display> {
		Some("an integer literal")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let literal = parser.try_update_with(|s| {
			if !s.starts_with(|ch: char| ch.is_ascii_digit()) {
				return Err(DecLiteralError::StartDigit);
			}
			*s = &s[1..];
			*s = s.trim_start_matches(|ch: char| ch.is_ascii_digit() || ch == '_');

			Ok(())
		})?;

		Ok(Self(literal))
	}
}
