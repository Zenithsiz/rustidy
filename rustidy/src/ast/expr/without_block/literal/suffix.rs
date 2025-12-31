//! Suffix

// Imports
use crate::{Format, Parse, ParseError, Parser, Print, ast::ident::IdentifierOrKeyword, parser::ParserError};

/// `SUFFIX`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Suffix(#[parse(with_tag = "skip:Whitespace")] IdentifierOrKeyword);

/// `SUFFIX_NO_E`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct SuffixNoE(IdentifierOrKeyword);

#[derive(Debug, ParseError)]
pub enum SuffixNoEError {
	#[parse_error(fmt = "Started with an `e` or `E`")]
	StartedWithE,

	#[parse_error(transparent)]
	IdentOrKeyword(ParserError<IdentifierOrKeyword>),
}

impl Parse for SuffixNoE {
	type Error = SuffixNoEError;

	#[coverage(off)]
	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let ident = parser
			.with_tag("skip:Whitespace", Parser::parse::<IdentifierOrKeyword>)
			.map_err(SuffixNoEError::IdentOrKeyword)?;
		if parser.str(&ident.1).starts_with(['e', 'E']) {
			return Err(SuffixNoEError::StartedWithE);
		}

		Ok(Self(ident))
	}
}
