//! Character literal

// Imports
use {
	crate::{Format, Parse, ParseError, Parser, ParserStr, Print, ast::whitespace::Whitespace, parser::ParserError},
	std::fmt,
};


/// `CHAR_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct CharLiteral(#[format(whitespace)] pub Whitespace, #[format(str)] pub ParserStr);

#[derive(Debug, ParseError)]
pub enum CharLiteralError {
	#[parse_error(fmt = "Expected `'`")]
	StartQuote,

	#[parse_error(fmt = "More than one character")]
	MoreThanOneChar,

	#[parse_error(fmt = "Expected `'` after `'`")]
	// Note: Not fatal because of lifetimes
	ExpectedEndQuote,

	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),
}

impl Parse for CharLiteral {
	type Error = CharLiteralError;

	fn name() -> Option<impl fmt::Display> {
		Some("a character literal")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let whitespace = parser.parse::<Whitespace>().map_err(CharLiteralError::Whitespace)?;
		let literal = parser.try_update_with(|s| {
			if !s.starts_with('\'') {
				return Err(CharLiteralError::StartQuote);
			}
			*s = &s[1..];

			// TODO: Parse escapes better?
			loop {
				let end = s.find('\'').ok_or(CharLiteralError::ExpectedEndQuote)?;

				// If this includes more than 1 character (or a newline), we can quit
				// TODO: This needs to work for escapes
				if s[..end].contains('\n') || (s[..end].chars().count() > 1 && !s[..end].contains('\\')) {
					return Err(CharLiteralError::MoreThanOneChar);
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
