//! Byte literal

// Imports
use {
	crate::ByteEscape,
	super::escape::ByteEscapeError,
	rustidy_format::{Format, Formattable},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{AstStr, Whitespace},
};

/// `BYTE_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a byte literal")]
#[parse(error(name = StartQuote, fmt = "Expected `b'`"))]
#[parse(error(name = ByteEscape(ByteEscapeError), transparent))]
#[parse(error(name = CharOrEscape, fmt = "Expected character or escape", fatal))]
#[parse(error(name = EndQuote, fmt = "Expected `'` after `b'`", fatal))]
pub struct ByteLiteral(
	pub Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub AstStr,
);

impl ByteLiteral {
	fn parse(s: &mut &str) -> Result<(), ByteLiteralError> {
		*s = s
			.strip_prefix("b\'")
			.ok_or(ByteLiteralError::StartQuote)?;
		match s.strip_prefix(
			|ch: char| ch.is_ascii() && !matches!(ch, '\'' | '\\' | '\n' | '\r' | '\t')
		) {
			Some(rest) => *s = rest,
			None => _ = rustidy_parse::try_parse_from_str(s, ByteEscape::parse)
				.map_err(ByteLiteralError::ByteEscape)?
				.ok()
				.ok_or(ByteLiteralError::CharOrEscape)?,
		}
		*s = s
			.strip_prefix('\'')
			.ok_or(ByteLiteralError::EndQuote)?;

		Ok(())
	}
}
