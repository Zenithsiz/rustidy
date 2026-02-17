//! Raw string literal

// Imports
use {
	super::Suffix,
	rustidy_format::{Format, Formattable},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::{AstStr, Whitespace},
};

/// `RAW_STRING_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a raw string literal")]
#[parse(error(name = StartR, fmt = "Expected `r`"))]
#[parse(error(name = StartQuote, fmt = "Expected `r\"`"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `r\"`", fatal))]
pub struct RawStringLiteral {
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	pub s:      AstStr,
	pub suffix: Option<Suffix>,
}

impl RawStringLiteral {
	fn parse(s: &mut &str) -> Result<(), RawStringLiteralError> {
		*s = s.strip_prefix('r').ok_or(RawStringLiteralError::StartR)?;

		let prefix_hash_len = s.find(|ch| ch != '#').ok_or(RawStringLiteralError::StartQuote)?;
		*s = &s[prefix_hash_len..];

		*s = s.strip_prefix('"').ok_or(RawStringLiteralError::StartQuote)?;

		let mut end_match = String::with_capacity(1 + prefix_hash_len);
		end_match.push('"');
		for _ in 0..prefix_hash_len {
			end_match.push('#');
		}

		let end = s.find(&end_match).ok_or(RawStringLiteralError::ExpectedEndQuote)?;
		*s = &s[end + end_match.len()..];

		Ok(())
	}
}
