//! Line remainder

// Imports
use {
	rustidy_format::{Format, Formattable},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::AstStr,
};

/// Characters remaining until the end of the line (including the newline if it exists)
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "remaining characters in line")]
pub struct RemainingLine(
	#[parse(update_with = Self::parse)]
	pub AstStr,
);

impl RemainingLine {
	fn parse(s: &mut &str) {
		*s = match s.find('\n') {
			Some(idx) => &s[idx + 1..],
			None => &s[s.len()..],
		};
	}
}

/// Characters remaining until the end of block comment
// TODO: This should not be in this file, or maybe this file should
//       just be merged with `attr.rs`.
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "remaining characters in block comment")]
#[parse(error(name = MissingCommentEnd, fmt = "Expected `*/` after `/*`", fatal))]
pub struct RemainingBlockComment(
	#[parse(try_update_with = Self::parse)]
	pub AstStr,
);

impl RemainingBlockComment {
	// TODO: Deduplicate this with `whitespace::BlockComment::parse`
	fn parse(s: &mut &str) -> Result<(), RemainingBlockCommentError> {
		let mut depth = 1;
		while depth != 0 {
			let close_idx = s.find("*/").ok_or(RemainingBlockCommentError::MissingCommentEnd)?;

			match s[..close_idx].find("/*") {
				Some(open_idx) => {
					*s = &s[open_idx + 2..];
					depth += 1;
				},
				None => {
					*s = &s[close_idx + 2..];
					depth -= 1;
				},
			}
		}

		Ok(())
	}
}
