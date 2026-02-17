//! Character literal

// Imports
use {
	super::escape::{AsciiEscapeError, QuoteEscapeError, UnicodeEscapeError},
	crate::{AsciiEscape, QuoteEscape, UnicodeEscape},
	rustidy_format::{Format, Formattable},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{AstStr, Whitespace},
};

/// `CHAR_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a character literal")]
#[parse(error(name = StartQuote, fmt = "Expected `'`"))]
#[parse(error(name = QuoteEscape(QuoteEscapeError), transparent))]
#[parse(error(name = AsciiEscape(AsciiEscapeError), transparent))]
#[parse(error(name = UnicodeEscape(UnicodeEscapeError), transparent))]
#[parse(error(name = CharOrEscape, fmt = "Expected character or escape", fatal))]
// Note: Not fatal because of lifetimes
#[parse(error(name = EndQuote, fmt = "Expected `'` after `'`"))]
pub struct CharLiteral(
	#[format(whitespace)] pub Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub AstStr,
);

impl CharLiteral {
	fn parse(s: &mut &str) -> Result<(), CharLiteralError> {
		*s = s.strip_prefix('\'').ok_or(CharLiteralError::StartQuote)?;
		match s.strip_prefix(|ch| !matches!(ch, '\'' | '\\' | '\n' | '\r' | '\t')) {
			Some(rest) => *s = rest,
			None => {
				// TODO: Better way to express this
				macro try_parse($Escape:ident) {
					rustidy_parse::try_parse_from_str(s, $Escape::parse)
						.map_err(CharLiteralError::$Escape)?
						.is_ok()
				}

				match () {
					() if try_parse!(QuoteEscape) => (),
					() if try_parse!(AsciiEscape) => (),
					() if try_parse!(UnicodeEscape) => (),

					() => return Err(CharLiteralError::CharOrEscape),
				}
			},
		}
		*s = s.strip_prefix('\'').ok_or(CharLiteralError::EndQuote)?;

		Ok(())
	}
}
