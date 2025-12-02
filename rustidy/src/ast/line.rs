//! Line remainder

// Imports
use crate::{AstStr, Format, Parse, ParseError, Parser, Print};

/// Characters remaining until the end of the line (including the newline if it exists)
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct RemainingLine(#[format(str)] pub AstStr);

#[derive(Debug, ParseError)]
#[parse_error(fmt = "Expected a line or EOF")]
pub struct RemainingLineError;

impl Parse for RemainingLine {
	type Error = RemainingLineError;

	fn name() -> Option<impl std::fmt::Display> {
		Some("remaining characters in line")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser
			.try_update_with(|s| {
				*s = match s.find('\n') {
					Some(idx) => &s[idx + 1..],
					None => &s[s.len()..],
				};

				Ok(())
			})
			.map(Self)
	}
}
