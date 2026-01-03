//! Byte string literal

// Imports
use {
	super::Suffix,
	crate::{
		Format,
		Parse,
		ParserStr,
		Print,
		ast::{
			expr::without_block::literal::{ByteEscape, StringContinue},
			whitespace::Whitespace,
		},
	},
};


/// `BYTE_STRING_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a byte string literal")]
#[parse(error(name = StartQuote, fmt = "Expected `b\"`"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `b\"`", fatal))]
pub struct ByteStringLiteral {
	#[format(whitespace)]
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub s:      ParserStr,
	pub suffix: Option<Suffix>,
}

impl ByteStringLiteral {
	fn parse(s: &mut &str) -> Result<(), ByteStringLiteralError> {
		*s = s.strip_prefix("b\"").ok_or(ByteStringLiteralError::StartQuote)?;

		loop {
			match s.strip_prefix(|ch: char| ch.is_ascii() && !matches!(ch, '"' | '\\' | '\r')) {
				Some(rest) => *s = rest,
				// TODO: We should report fatal errors from here
				None =>
					if ByteEscape::parse(s)
						.ok()
						.or_else(|| StringContinue::parse(s).ok())
						.is_none()
					{
						break;
					},
			}
		}

		*s = s.strip_prefix('"').ok_or(ByteStringLiteralError::ExpectedEndQuote)?;

		Ok(())
	}
}
