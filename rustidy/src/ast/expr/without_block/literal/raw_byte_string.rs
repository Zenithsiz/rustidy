//! Raw byte string literal

// Imports
use {
	super::Suffix,
	crate::{Format, Print, ast::whitespace::Whitespace},
	rustidy_parse::{Parse, ParserStr},
};

/// `RAW_BYTE_STRING_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a raw byte string literal")]
#[parse(error(name = StartR, fmt = "Expected `br`"))]
#[parse(error(name = StartQuote, fmt = "Expected `br\"`"))]
#[parse(error(name = NonAscii, fmt = "Found non-ascii characters"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `br\"`", fatal))]
pub struct RawByteStringLiteral {
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	pub s:      ParserStr,
	pub suffix: Option<Suffix>,
}

impl RawByteStringLiteral {
	fn parse(s: &mut &str) -> Result<(), RawByteStringLiteralError> {
		*s = s.strip_prefix("br").ok_or(RawByteStringLiteralError::StartR)?;

		let prefix_hash_len = s.find(|ch| ch != '#').ok_or(RawByteStringLiteralError::StartQuote)?;
		*s = &s[prefix_hash_len..];

		*s = s.strip_prefix('"').ok_or(RawByteStringLiteralError::StartQuote)?;

		let mut end_match = String::with_capacity(1 + prefix_hash_len);
		end_match.push('"');
		for _ in 0..prefix_hash_len {
			end_match.push('#');
		}

		let end = s.find(&end_match).ok_or(RawByteStringLiteralError::ExpectedEndQuote)?;
		if let Some(idx) = s[..end].find(|ch: char| !ch.is_ascii()) {
			*s = &s[idx..];
			return Err(RawByteStringLiteralError::NonAscii);
		}

		*s = &s[end + end_match.len()..];

		Ok(())
	}
}
