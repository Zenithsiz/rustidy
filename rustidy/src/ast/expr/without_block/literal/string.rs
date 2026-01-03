//! String literal

// Imports
use {
	super::Suffix,
	crate::{Format, Parse, ParserStr, Print, ast::whitespace::Whitespace},
};


/// `STRING_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a string literal")]
#[parse(error(name = StartQuote, fmt = "Expected `\"`"))]
#[parse(error(name = ExpectedEndQuote, fmt = "Expected `\"` after `\"`", fatal))]
pub struct StringLiteral {
	#[format(whitespace)]
	pub ws:     Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub s:      ParserStr,
	pub suffix: Option<Suffix>,
}

impl StringLiteral {
	fn parse(s: &mut &str) -> Result<(), StringLiteralError> {
		if !s.starts_with('"') {
			return Err(StringLiteralError::StartQuote);
		}
		*s = &s[1..];

		// TODO: Parse escapes better?
		loop {
			let end = s.find('"').ok_or(StringLiteralError::ExpectedEndQuote)?;
			let is_escape = s[..end].ends_with('\\') && !s[..end].ends_with("\\\\");
			*s = &s[end + 1..];
			if !is_escape {
				break;
			}
		}

		Ok(())
	}
}
