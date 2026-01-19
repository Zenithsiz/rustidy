//! Suffix

// Imports
use {
	rustidy_ast_util::IdentifierOrKeyword,
	rustidy_format::Format,
	rustidy_parse::{Parse, Parser},
	rustidy_print::Print,
};

/// `SUFFIX`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Suffix(#[parse(with_tag = "skip:Whitespace")] IdentifierOrKeyword);

/// `SUFFIX_NO_E`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = StartedWithE, fmt = "Started with an `e` or `E`"))]
#[parse(and_try_with = Self::check_no_e)]
pub struct SuffixNoE(pub Suffix);

impl SuffixNoE {
	fn check_no_e(&self, parser: &mut Parser) -> Result<(), SuffixNoEError> {
		if parser.str(&self.0.0.1).starts_with(['e', 'E']) {
			return Err(SuffixNoEError::StartedWithE);
		}

		Ok(())
	}
}
