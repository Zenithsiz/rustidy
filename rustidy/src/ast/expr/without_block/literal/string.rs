//! String literal

// Imports
use {
	super::Suffix,
	crate::{Format, Parse, ParseError, Parser, ParserStr, Print, ast::whitespace::Whitespace, parser::ParserError},
	std::fmt,
};


/// `STRING_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct StringLiteral {
	#[format(whitespace)]
	pub ws:     Whitespace,
	#[format(str)]
	pub s:      ParserStr,
	pub suffix: Option<Suffix>,
}

#[derive(Debug, ParseError)]
pub enum StringLiteralError {
	#[parse_error(fmt = "Expected `\"`")]
	StartQuote,

	#[parse_error(fmt = "Expected `\"` after `\"`")]
	#[parse_error(fatal)]
	ExpectedEndQuote,

	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),

	#[parse_error(transparent)]
	#[parse_error(fatal)]
	Suffix(ParserError<Option<Suffix>>),
}

impl Parse for StringLiteral {
	type Error = StringLiteralError;

	fn name() -> Option<impl fmt::Display> {
		Some("a string literal")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let ws = parser.parse::<Whitespace>().map_err(StringLiteralError::Whitespace)?;
		let s = parser.try_update_with(|s| {
			if !s.starts_with('"') {
				return Err(StringLiteralError::StartQuote);
			}
			*s = &s[1..];

			// TODO: Parse escapes better?
			loop {
				let end = s.find('"').ok_or(StringLiteralError::ExpectedEndQuote)?;
				let is_escape = s[..end].ends_with('\\') && !s[..end].ends_with("\\\\");
				*s = &s[end + 1..];
				if !is_escape {
					break;
				}
			}

			Ok(())
		})?;
		let suffix = parser.parse().map_err(StringLiteralError::Suffix)?;

		Ok(Self { ws, s, suffix })
	}
}
