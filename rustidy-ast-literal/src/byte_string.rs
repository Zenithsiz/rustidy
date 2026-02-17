//! Byte string literal

// Imports
use {
	super::{
		Suffix,
		escape::{ByteEscapeError, StringContinueError},
	},
	crate::{ByteEscape, StringContinue},
	rustidy_format::{Format, Formattable},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{AstStr, Whitespace},
};


/// `BYTE_STRING_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a byte string literal")]
#[parse(error(name = StartQuote, fmt = "Expected `b\"`"))]
#[parse(error(name = ByteEscape(ByteEscapeError), transparent))]
#[parse(error(name = StringContinue(StringContinueError), transparent))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `b\"`", fatal))]
pub struct ByteStringLiteral {
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	pub s:      AstStr,
	pub suffix: Option<Suffix>,
}

impl ByteStringLiteral {
	fn parse(s: &mut &str) -> Result<(), ByteStringLiteralError> {
		*s = s.strip_prefix("b\"").ok_or(ByteStringLiteralError::StartQuote)?;

		loop {
			match s.strip_prefix(|ch: char| ch.is_ascii() && !matches!(ch, '"' | '\\' | '\r')) {
				Some(rest) => *s = rest,
				None => {
					macro try_parse($Escape:ident) {
						rustidy_parse::try_parse_from_str(s, $Escape::parse)
							.map_err(ByteStringLiteralError::$Escape)?
							.is_ok()
					}

					match () {
						() if try_parse!(ByteEscape) => (),
						() if try_parse!(StringContinue) => (),

						() => break,
					}
				},
			}
		}

		*s = s.strip_prefix('"').ok_or(ByteStringLiteralError::ExpectedEndQuote)?;

		Ok(())
	}
}
