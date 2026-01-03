//! Byte literal

// Imports
use crate::{Format, Parse, ParserStr, Print, ast::whitespace::Whitespace};

/// `BYTE_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a byte literal")]
#[parse(error(name = StartQuote, fmt = "Expected `b'`"))]
#[parse(error(name = MoreThanOneByte, fmt = "More than one byte"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `'` after `b'`", fatal))]
pub struct ByteLiteral(
	#[format(whitespace)] pub Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl ByteLiteral {
	fn parse(s: &mut &str) -> Result<(), ByteLiteralError> {
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
	}
}
