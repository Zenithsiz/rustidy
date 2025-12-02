//! Raw string literal

// Imports
use {
	super::Suffix,
	crate::{AstStr, Format, Parse, ParseError, Parser, Print, ast::whitespace::Whitespace, parser::ParserError},
	std::fmt,
};


/// `RAW_STRING_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct RawStringLiteral {
	#[format(str)]
	pub s:      AstStr,
	pub suffix: Option<Suffix>,
	#[format(whitespace)]
	pub ws:     Whitespace,
}

#[derive(Debug, ParseError)]
pub enum RawStringLiteralError {
	#[parse_error(fmt = "Expected `r`")]
	StartR,

	#[parse_error(fmt = "Expected `r\"`")]
	StartQuote,

	#[parse_error(fmt = "Expected `\"` after `r\"`")]
	#[parse_error(fatal)]
	ExpectedEndQuote,

	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),

	#[parse_error(transparent)]
	#[parse_error(fatal)]
	Suffix(ParserError<Option<Suffix>>),
}

impl Parse for RawStringLiteral {
	type Error = RawStringLiteralError;

	fn name() -> Option<impl fmt::Display> {
		Some("a string literal")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let s = parser.try_update_with(|s| {
			if !s.starts_with('r') {
				return Err(RawStringLiteralError::StartR);
			}
			*s = &s[1..];

			let prefix_hash_len = s.find(|ch| ch != '#').ok_or(RawStringLiteralError::StartQuote)?;
			*s = &s[prefix_hash_len..];

			if !s.starts_with('"') {
				return Err(RawStringLiteralError::StartQuote);
			}
			*s = &s[1..];

			let mut end_match = String::with_capacity(1 + prefix_hash_len);
			end_match.push('"');
			for _ in 0..prefix_hash_len {
				end_match.push('#');
			}

			let end = s.find(&end_match).ok_or(RawStringLiteralError::ExpectedEndQuote)?;
			*s = &s[end + end_match.len()..];

			Ok(())
		})?;
		let suffix = parser.parse().map_err(RawStringLiteralError::Suffix)?;

		let ws = parser
			.parse::<Whitespace>()
			.map_err(RawStringLiteralError::Whitespace)?;

		Ok(Self { s, suffix, ws })
	}
}
