//! Line remainder

// Imports
use crate::{Format, Parse, ParserStr, Print};

/// Characters remaining until the end of the line (including the newline if it exists)
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "remaining characters in line")]
pub struct RemainingLine(
	#[parse(update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

impl RemainingLine {
	fn parse(s: &mut &str) {
		*s = match s.find('\n') {
			Some(idx) => &s[idx + 1..],
			None => &s[s.len()..],
		};
	}
}
