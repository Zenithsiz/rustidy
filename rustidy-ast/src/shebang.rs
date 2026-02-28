//! Shebang

// Imports
use {format::{Format, Formattable}, parse::Parse, print::Print, util::AstStr};

/// Shebang
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a shebang")]
#[parse(error(name = Shebang, fmt = "Expected a `#!`"))]
#[format(no_prefix_ws)]
pub struct Shebang(#[parse(try_update_with = Self::parse)] #[format(str)] pub AstStr);

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
