//! Shebang

// Imports
use crate::{Format, Parse, ParserStr, Print};

/// Shebang
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a shebang")]
#[parse(error(name = Shebang, fmt = "Expected a `#!`"))]
pub struct Shebang(#[parse(try_update_with = Self::parse)] pub ParserStr);

impl Shebang {
	fn parse(s: &mut &str) -> Result<(), ShebangError> {
		if !s.starts_with("#!") || s.starts_with("#![") {
			return Err(ShebangError::Shebang);
		}

		*s = match s.find('\n') {
			Some(idx) => &s[idx + 1..],
			None => &s[s.len()..],
		};

		Ok(())
	}
}
