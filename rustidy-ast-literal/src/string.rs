//! String literal

// Imports
use {
	super::Suffix,
	crate::{AsciiEscape, QuoteEscape, StringContinue, UnicodeEscape},
	rustidy_ast_util::Whitespace,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{AstStr, Config},
	std::borrow::Cow,
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
	// TODO: Split this into the two quotes and the contents?
	#[parse(try_update_with = Self::parse)]
	pub s:      AstStr,
	pub suffix: Option<Suffix>,
}

impl StringLiteral {
	/// Returns the contents of this string.
	///
	/// This doesn't include the quotes or suffix
	#[must_use]
	pub fn contents<'input>(&self, input: &'input str, config: &Config) -> Cow<'input, str> {
		let s = self.s.str(input, config);
		match s {
			Cow::Borrowed(s) => Cow::Borrowed(&s[..s.len() - 1][1..]),
			Cow::Owned(mut s) => {
				s.pop();
				s.remove(0);

				Cow::Owned(s)
			},
		}
	}

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
