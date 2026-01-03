//! Escapes

// Imports
use crate::{Format, Parse, ParserStr, Print};

/// `QUOTE_ESCAPE`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Escape, fmt = "Expected `\\'` or `\\\"`"))]
pub struct QuoteEscape(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl QuoteEscape {
	pub fn parse(s: &mut &str) -> Result<(), QuoteEscapeError> {
		s.strip_prefix("\\'")
			.or_else(|| s.strip_prefix("\\\""))
			.map(|rest| *s = rest)
			.ok_or(QuoteEscapeError::Escape)
	}
}

/// `ASCII_ESCAPE`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Escape, fmt = "Expected `\\xXX`, `\\n`, `\\r`, `\\t`, `\\\\` or `\\0`"))]
#[parse(error(name = Octal, fmt = "Expected octal digit", fatal))]
#[parse(error(name = Hex, fmt = "Expected hex digit", fatal))]
pub struct AsciiEscape(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl AsciiEscape {
	pub fn parse(s: &mut &str) -> Result<(), AsciiEscapeError> {
		if let Some(rest) = s.strip_prefix("\\x") {
			*s = rest;
			*s = s
				.strip_prefix(|ch: char| ch.is_ascii_octdigit())
				.ok_or(AsciiEscapeError::Octal)?;
			*s = s
				.strip_prefix(|ch: char| ch.is_ascii_hexdigit())
				.ok_or(AsciiEscapeError::Hex)?;
			return Ok(());
		}

		s.strip_prefix("\\n")
			.or_else(|| s.strip_prefix("\\r"))
			.or_else(|| s.strip_prefix("\\t"))
			.or_else(|| s.strip_prefix("\\\\"))
			.or_else(|| s.strip_prefix("\\0"))
			.map(|rest| *s = rest)
			.ok_or(AsciiEscapeError::Escape)
	}
}

/// `BYTE_ESCAPE`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Escape, fmt = "Expected `\\xXX`, `\\n`, `\\r`, `\\t`, `\\\\`, `\\0`, `'` or `\"`"))]
#[parse(error(name = Hex, fmt = "Expected hex digit"))]
pub struct ByteEscape(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl ByteEscape {
	pub fn parse(s: &mut &str) -> Result<(), ByteEscapeError> {
		if let Some(rest) = s.strip_prefix("\\x") {
			*s = rest;
			*s = s
				.strip_prefix(|ch: char| ch.is_ascii_hexdigit())
				.ok_or(ByteEscapeError::Hex)?;
			*s = s
				.strip_prefix(|ch: char| ch.is_ascii_hexdigit())
				.ok_or(ByteEscapeError::Hex)?;
			return Ok(());
		}

		s.strip_prefix("\\n")
			.or_else(|| s.strip_prefix("\\r"))
			.or_else(|| s.strip_prefix("\\t"))
			.or_else(|| s.strip_prefix("\\\\"))
			.or_else(|| s.strip_prefix("\\0"))
			.or_else(|| s.strip_prefix("\\'"))
			.or_else(|| s.strip_prefix("\\\""))
			.map(|rest| *s = rest)
			.ok_or(ByteEscapeError::Escape)
	}
}

/// `UNICODE_ESCAPE`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Escape, fmt = "Expected `\\u{XXXXX}`"))]
#[parse(error(name = TooManyDigits, fmt = "Expected at most 6 digits", fatal))]
pub struct UnicodeEscape(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl UnicodeEscape {
	pub fn parse(s: &mut &str) -> Result<(), UnicodeEscapeError> {
		*s = s.strip_prefix("\\u{").ok_or(UnicodeEscapeError::Escape)?;

		for _ in 0..6 {
			match s.strip_prefix(|ch: char| ch.is_ascii_hexdigit()) {
				Some(rest) => {
					*s = rest;
					*s = s.trim_start_matches('_');
				},
				None => break,
			}
		}
		if s.starts_with(|ch: char| ch.is_ascii_hexdigit()) {
			return Err(UnicodeEscapeError::TooManyDigits);
		}

		*s = s.strip_prefix('}').ok_or(UnicodeEscapeError::Escape)?;

		Ok(())
	}
}

/// `STRING_CONTINUE`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Escape, fmt = "Expected `\\` and a newline"))]
pub struct StringContinue(
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl StringContinue {
	pub fn parse(s: &mut &str) -> Result<(), StringContinueError> {
		*s = s.strip_prefix("\\\n").ok_or(StringContinueError::Escape)?;
		Ok(())
	}
}
