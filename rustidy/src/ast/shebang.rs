//! Shebang

// Imports
use crate::{Format, Parse, ParseError, Parser, ParserStr, Print};

/// Shebang
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct Shebang(#[format(str)] pub ParserStr);

#[derive(Debug, ParseError)]
#[parse_error(fmt = "Expected a `#!`")]
pub struct ShebangError;

impl Parse for Shebang {
	type Error = ShebangError;

	fn name() -> Option<impl std::fmt::Display> {
		Some("a shebang")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		parser
			.try_update_with(|s| {
				if !s.starts_with("#!") || s.starts_with("#![") {
					return Err(ShebangError);
				}

				*s = match s.find('\n') {
					Some(idx) => &s[idx + 1..],
					None => &s[s.len()..],
				};

				Ok(())
			})
			.map(Self)
	}
}
