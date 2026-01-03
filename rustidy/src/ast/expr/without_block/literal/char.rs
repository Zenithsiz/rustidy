//! Character literal

// Imports
use crate::{Format, Parse, ParserStr, Print, ast::whitespace::Whitespace};

/// `CHAR_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a character literal")]
#[parse(error(name = StartQuote, fmt = "Expected `'`"))]
#[parse(error(name = MoreThanOneChar, fmt = "More than one character"))]
// Note: Not fatal because of lifetimes
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `'` after `'`"))]
pub struct CharLiteral(
	#[format(whitespace)] pub Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl CharLiteral {
	fn parse(s: &mut &str) -> Result<(), CharLiteralError> {
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
	}
}
