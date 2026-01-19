//! C string literal

// Imports
use {
	super::{
		Suffix,
		escape::{NonNulByteEscapeError, NonNulUnicodeEscapeError, StringContinueError},
	},
	crate::{NonNulByteEscape, NonNulUnicodeEscape, StringContinue},
	rustidy_ast_util::Whitespace,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::AstStr,
};

/// `C_STRING_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a C string literal")]
#[parse(error(name = StartQuote, fmt = "Expected `c\"`"))]
#[parse(error(name = NonNulByteEscape(NonNulByteEscapeError), transparent))]
#[parse(error(name = NonNulUnicodeEscape(NonNulUnicodeEscapeError), transparent))]
#[parse(error(name = StringContinue(StringContinueError), transparent))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `c\"`", fatal))]
pub struct CStringLiteral {
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	pub s:      AstStr,
	pub suffix: Option<Suffix>,
}

impl CStringLiteral {
	fn parse(s: &mut &str) -> Result<(), CStringLiteralError> {
		*s = s.strip_prefix("c\"").ok_or(CStringLiteralError::StartQuote)?;

		loop {
			match s.strip_prefix(|ch: char| !matches!(ch, '"' | '\\' | '\r' | '\0')) {
				Some(rest) => *s = rest,
				None => {
					macro try_parse($Escape:ident) {
						rustidy_parse::try_parse_from_str(s, $Escape::parse)
							.map_err(CStringLiteralError::$Escape)?
							.is_ok()
					}

					match () {
						() if try_parse!(NonNulByteEscape) => (),
						() if try_parse!(NonNulUnicodeEscape) => (),
						() if try_parse!(StringContinue) => (),

						() => break,
					}
				},
			}
		}

		*s = s.strip_prefix('"').ok_or(CStringLiteralError::ExpectedEndQuote)?;

		Ok(())
	}
}
