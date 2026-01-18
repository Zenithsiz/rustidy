//! String literal

// Imports
use {
	super::Suffix,
	crate::{
		Format,
		Print,
		ast::{
			expr::without_block::literal::{AsciiEscape, QuoteEscape, StringContinue, UnicodeEscape},
			whitespace::Whitespace,
		},
	},
	rustidy_parse::Parse,
	rustidy_util::ParserStr,
};

/// `STRING_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a string literal")]
#[parse(error(name = StartQuote, fmt = "Expected `\"`"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `\"`", fatal))]
pub struct StringLiteral {
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	pub s:      ParserStr,
	pub suffix: Option<Suffix>,
}

impl StringLiteral {
	fn parse(s: &mut &str) -> Result<(), StringLiteralError> {
		*s = s.strip_prefix('"').ok_or(StringLiteralError::StartQuote)?;

		loop {
			match s.strip_prefix(|ch| !matches!(ch, '"' | '\\' | '\r')) {
				Some(rest) => *s = rest,
				// TODO: We should report fatal errors from here
				None =>
					if QuoteEscape::parse(s)
						.ok()
						.or_else(|| AsciiEscape::parse(s).ok())
						.or_else(|| UnicodeEscape::parse(s).ok())
						.or_else(|| StringContinue::parse(s).ok())
						.is_none()
					{
						break;
					},
			}
		}

		*s = s.strip_prefix('"').ok_or(StringLiteralError::ExpectedEndQuote)?;

		Ok(())
	}
}
