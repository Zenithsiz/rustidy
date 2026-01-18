//! Raw C string literal

// Imports
use {
	super::Suffix,
	crate::{Format, Print, ast::whitespace::Whitespace},
	rustidy_parse::Parse,
	rustidy_util::AstStr,
};

/// `RAW_C_STRING_LITERAL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a raw C string literal")]
#[parse(error(name = StartR, fmt = "Expected `cr`"))]
#[parse(error(name = StartQuote, fmt = "Expected `cr\"`"))]
#[parse(error(name = Nul, fmt = "Found a nul character"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `cr\"`", fatal))]
pub struct RawCStringLiteral {
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	pub s:      AstStr,
	pub suffix: Option<Suffix>,
}

impl RawCStringLiteral {
	fn parse(s: &mut &str) -> Result<(), RawCStringLiteralError> {
		*s = s.strip_prefix("cr").ok_or(RawCStringLiteralError::StartR)?;

		let prefix_hash_len = s.find(|ch| ch != '#').ok_or(RawCStringLiteralError::StartQuote)?;
		*s = &s[prefix_hash_len..];

		*s = s.strip_prefix('"').ok_or(RawCStringLiteralError::StartQuote)?;

		let mut end_match = String::with_capacity(1 + prefix_hash_len);
		end_match.push('"');
		for _ in 0..prefix_hash_len {
			end_match.push('#');
		}

		let end = s.find(&end_match).ok_or(RawCStringLiteralError::ExpectedEndQuote)?;
		if let Some(idx) = s[..end].find('\0') {
			*s = &s[idx..];
			return Err(RawCStringLiteralError::Nul);
		}

		*s = &s[end + end_match.len()..];

		Ok(())
	}
}
