//! Shebang

// Imports
use {
	crate::{Format, Print},
	rustidy_parse::Parse,
	rustidy_util::AstStr,
};

/// Shebang
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a shebang")]
#[parse(error(name = Shebang, fmt = "Expected a `#!`"))]
pub struct Shebang(#[parse(try_update_with = Self::parse)] pub AstStr);

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
