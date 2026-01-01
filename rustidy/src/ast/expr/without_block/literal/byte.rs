//! Byte literal

// Imports
use {
	crate::{Format, Parse, ParseError, Parser, ParserStr, Print, ast::whitespace::Whitespace, parser::ParserError},
	std::fmt,
};


/// `BYTE_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct ByteLiteral(#[format(whitespace)] pub Whitespace, #[format(str)] pub ParserStr);

#[derive(Debug, ParseError)]
pub enum ByteLiteralError {
	#[parse_error(fmt = "Expected `b'`")]
	StartQuote,

	#[parse_error(fmt = "More than one byte")]
	MoreThanOneByte,

	#[parse_error(fmt = "Expected `'` after `b'`")]
	#[parse_error(fatal)]
	ExpectedEndQuote,

	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),
}

impl Parse for ByteLiteral {
	type Error = ByteLiteralError;

	fn name() -> Option<impl fmt::Display> {
		Some("a byte literal")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let whitespace = parser.parse::<Whitespace>().map_err(ByteLiteralError::Whitespace)?;
		let literal = parser.try_update_with(|s| {
			if !s.starts_with("b\'") {
				return Err(ByteLiteralError::StartQuote);
			}
			*s = &s[2..];

			// TODO: Parse escapes better?
			loop {
				let end = s.find('\'').ok_or(ByteLiteralError::ExpectedEndQuote)?;

				// If this includes more than 1 byte, we can quit
				// TODO: This needs to work for escapes
				if s[..end].len() > 1 && !s[..end].contains('\\') {
					return Err(ByteLiteralError::MoreThanOneByte);
				}

				let is_escape = s[..end].ends_with('\\') && !s[..end].ends_with("\\\\");
				*s = &s[end + 1..];
				if !is_escape {
					break;
				}
			}

			Ok(())
		})?;

		Ok(Self(whitespace, literal))
	}
}
