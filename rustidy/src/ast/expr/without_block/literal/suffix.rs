//! Suffix

// Imports
use crate::{Format, Parse, ParseError, Parser, Print, ast::ident::IdentifierOrKeywordRaw, parser::ParserError};

/// `SUFFIX`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Suffix(IdentifierOrKeywordRaw);

/// `SUFFIX_NO_E`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct SuffixNoE(IdentifierOrKeywordRaw);

#[derive(Debug, ParseError)]
pub enum SuffixNoEError {
	#[parse_error(fmt = "Started with an `e` or `E`")]
	StartedWithE,

	#[parse_error(transparent)]
	IdentOrKeyword(ParserError<IdentifierOrKeywordRaw>),
}

impl Parse for SuffixNoE {
	type Error = SuffixNoEError;

	#[coverage(off)]
	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let ident = parser
			.parse::<IdentifierOrKeywordRaw>()
			.map_err(SuffixNoEError::IdentOrKeyword)?;
		if ident.0.as_str(parser).starts_with(['e', 'E']) {
			return Err(SuffixNoEError::StartedWithE);
		}

		Ok(Self(ident))
	}
}
