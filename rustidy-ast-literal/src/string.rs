//! String literal

// Imports
use {
	crate::{AsciiEscape, QuoteEscape, StringContinue, UnicodeEscape},
	super::Suffix,
	format::{Format, Formattable},
	parse::Parse,
	print::Print,
	std::borrow::Cow,
	util::{AstStr, Whitespace},
};

/// `STRING_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a string literal")]
#[parse(error(name = StartQuote, fmt = "Expected `\"`"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `\"`", fatal))]
pub struct StringLiteral {
	pub ws:     Whitespace,
	// TODO: Split this into the two quotes and the contents?
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub s:      AstStr,
	#[format(prefix_ws = ())]
	pub suffix: Option<Suffix>,
}

impl StringLiteral {
	/// Returns the contents of this string.
	///
	/// Doesn't include the quotes or suffix, and resolves any escapes
	#[must_use]
	pub fn contents(&self) -> Cow<'_, str> {
		let mut s = self.s.str();

		// Remove the quotes
		match &mut s {
			Cow::Borrowed(s) => *s = &s[..s.len() - 1][1..],
			Cow::Owned(s) => {
				s.pop();
				s.remove(0);
			},
		}

		// Find and resolve any escapes
		let mut cur_idx = 0;
		while let Some(idx) = s[cur_idx..].find('\\') {
			let s = Cow::to_mut(&mut s);
			let replace_with = |s: &mut String, replacement| s
				.replace_range(idx..idx + 2, replacement);
			match s[idx + 1..].chars().next() {
				Some('n') => replace_with(s, "\n"),
				Some('r') => replace_with(s, "\r"),
				Some('t') => replace_with(s, "\t"),
				Some('\\') => replace_with(s, "\\"),
				Some('0') => replace_with(s, "\0"),
				Some('\'') => replace_with(s, "\'"),
				Some('"') => replace_with(s, "\""),
				Some('\n') => replace_with(s, ""),

				// TODO: Support hex and unicode escapes.
				Some(escape_ch) => {
					tracing::warn!("Unknown escape: '\\{escape_ch}'",);
					cur_idx = idx + 1;
				},
				None => {
					tracing::warn!("Expected character after `\\`, found EOF");
					break;
				},
			}
		}


		s
	}

	fn parse(s: &mut &str) -> Result<(), StringLiteralError> {
		*s = s
			.strip_prefix('"')
			.ok_or(StringLiteralError::StartQuote)?;

		loop {
			match s
				.strip_prefix(|ch| !matches!(ch, '"' | '\\' | '\r')) {
				Some(rest) => *s = rest,
				// TODO: We should report fatal errors from here
				None => if QuoteEscape::parse(s)
					.ok()
					.or_else(|| AsciiEscape::parse(s).ok())
					.or_else(|| UnicodeEscape::parse(s).ok())
					.or_else(|| StringContinue::parse(s).ok())
					.is_none() {
					break;
				},
			}
		}

		*s = s
			.strip_prefix('"')
			.ok_or(StringLiteralError::ExpectedEndQuote)?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use {super::*, parse::Parser};

	#[test]
	fn contents() {
		let cases = [
			("\"\"", ""),
			("\"\\n\"", "\n"),
			("\"\\r\"", "\r"),
			("\"\\t\"", "\t"),
			("\"\\\\\"", "\\"),
			("\"\\0\"", "\0"),
			("\"\\'\"", "'"),
			("\"\\\"\"", "\""),
			("\"\\\n\"", ""),
			("\"012\\r345\\t678\"", "012\r345\t678"),
		];

		for (input, contents_expected) in cases {
			let mut parser = Parser::new(input);
			let literal = parser
				.parse::<StringLiteral>()
				.unwrap_or_else(
					|err| panic!("Unable to parse input case {input:?}: {err:?}")
				);

			let contents_found = literal.contents();
			assert_eq!(
				contents_found, contents_expected,
				"Found wrong contents for string {input:?}"
			);
		}
	}
}
