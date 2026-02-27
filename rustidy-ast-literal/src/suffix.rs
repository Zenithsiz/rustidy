//! Suffix

// Imports
use {
	crate::IdentifierOrKeyword,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, Parser, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `SUFFIX`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(no_prefix_ws)]
pub struct Suffix(
	#[parse(with_tag = ParserTag::SkipWhitespace)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	IdentifierOrKeyword,
);

/// `SUFFIX_NO_E`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = StartedWithE, fmt = "Started with an `e` or `E`"))]
#[parse(and_try_with = Self::check_no_e)]
#[format(no_prefix_ws)]
pub struct SuffixNoE(pub Suffix);

impl SuffixNoE {
	fn check_no_e(&self, _parser: &mut Parser) -> Result<(), SuffixNoEError> {
		if self.0.0.1.str().starts_with(['e', 'E']) {
			return Err(SuffixNoEError::StartedWithE);
		}

		Ok(())
	}
}
