//! Character literal

// Imports
use crate::{
	Format,
	Parse,
	ParserStr,
	Print,
	ast::{
		expr::without_block::literal::{AsciiEscape, QuoteEscape, UnicodeEscape},
		whitespace::Whitespace,
	},
};

/// `CHAR_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a character literal")]
#[parse(error(name = StartQuote, fmt = "Expected `'`"))]
#[parse(error(name = CharOrEscape, fmt = "Expected character or escape", fatal))]
// Note: Not fatal because of lifetimes
#[parse(error(name = EndQuote, fmt = "Expected `'` after `'`"))]
pub struct CharLiteral(
	#[format(whitespace)] pub Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl CharLiteral {
	fn parse(s: &mut &str) -> Result<(), CharLiteralError> {
		*s = s.strip_prefix('\'').ok_or(CharLiteralError::StartQuote)?;
		match s.strip_prefix(|ch| !matches!(ch, '\'' | '\\' | '\n' | '\r' | '\t')) {
			Some(rest) => *s = rest,
			// TODO: We should report fatal errors from here
			None => QuoteEscape::parse(s)
				.ok()
				.or_else(|| AsciiEscape::parse(s).ok())
				.or_else(|| UnicodeEscape::parse(s).ok())
				.ok_or(CharLiteralError::CharOrEscape)?,
		}
		*s = s.strip_prefix('\'').ok_or(CharLiteralError::EndQuote)?;

		Ok(())
	}
}
