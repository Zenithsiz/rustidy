//! Integer literal

// Imports
use {
	super::SuffixNoE,
	crate::{AstStr, Format, Parse, ParseError, Parser, Print, ast::whitespace::Whitespace, parser::ParserError},
	std::fmt,
};


/// `INTEGER_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IntegerLiteral {
	inner:  IntegerLiteralInner,
	suffix: Option<SuffixNoE>,
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
pub struct DecLiteral(#[format(str)] pub AstStr, #[format(whitespace)] pub Whitespace);

#[derive(Debug, ParseError)]
pub enum DecLiteralError {
	#[parse_error(fmt = "Expected 0-9")]
	StartDigit,

	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),
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

			// TODO: Allow escapes
			let end = s
				.find(|ch: char| !char::is_ascii_digit(&ch) && ch != '_')
				.unwrap_or(s.len());
			*s = &s[end..];

			Ok(())
		})?;

		let whitespace = parser.parse::<Whitespace>().map_err(DecLiteralError::Whitespace)?;

		Ok(Self(literal, whitespace))
	}
}
