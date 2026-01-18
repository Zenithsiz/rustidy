//! Byte literal

// Imports
use {
	super::escape::ByteEscapeError,
	crate::{
		Format,
		Print,
		ast::{expr::without_block::literal::ByteEscape, whitespace::Whitespace},
	},
	rustidy_parse::{Parse, ParserStr},
};

/// `BYTE_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a byte literal")]
#[parse(error(name = StartQuote, fmt = "Expected `b'`"))]
#[parse(error(name = ByteEscape(ByteEscapeError), transparent))]
#[parse(error(name = CharOrEscape, fmt = "Expected character or escape", fatal))]
#[parse(error(name = EndQuote, fmt = "Expected `'` after `b'`", fatal))]
pub struct ByteLiteral(pub Whitespace, #[parse(try_update_with = Self::parse)] pub ParserStr);

impl ByteLiteral {
	fn parse(s: &mut &str) -> Result<(), ByteLiteralError> {
		*s = s.strip_prefix("b\'").ok_or(ByteLiteralError::StartQuote)?;
		match s.strip_prefix(|ch: char| ch.is_ascii() && !matches!(ch, '\'' | '\\' | '\n' | '\r' | '\t')) {
			Some(rest) => *s = rest,
			None =>
				_ = rustidy_parse::try_parse_from_str(s, ByteEscape::parse)
					.map_err(ByteLiteralError::ByteEscape)?
					.ok()
					.ok_or(ByteLiteralError::CharOrEscape)?,
		}
		*s = s.strip_prefix('\'').ok_or(ByteLiteralError::EndQuote)?;

		Ok(())
	}
}
