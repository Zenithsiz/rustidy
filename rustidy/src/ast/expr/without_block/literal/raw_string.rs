//! Raw string literal

// Imports
use {
	super::Suffix,
	crate::{Format, Parse, ParserStr, Print, ast::whitespace::Whitespace},
};


/// `RAW_STRING_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a raw string literal")]
#[parse(error(name = StartR, fmt = "Expected `r`"))]
#[parse(error(name = StartQuote, fmt = "Expected `r\"`"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `r\"`", fatal))]
pub struct RawStringLiteral {
	#[format(whitespace)]
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub s:      ParserStr,
	pub suffix: Option<Suffix>,
}

impl RawStringLiteral {
	fn parse(s: &mut &str) -> Result<(), RawStringLiteralError> {
		if !s.starts_with('r') {
			return Err(RawStringLiteralError::StartR);
		}
		*s = &s[1..];

		let prefix_hash_len = s.find(|ch| ch != '#').ok_or(RawStringLiteralError::StartQuote)?;
		*s = &s[prefix_hash_len..];

		if !s.starts_with('"') {
			return Err(RawStringLiteralError::StartQuote);
		}
		*s = &s[1..];

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
