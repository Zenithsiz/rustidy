//! Identifier

// Modules
pub mod ident_or_keyword;
pub mod keyword;

// Exports
pub use self::{ident_or_keyword::IdentifierOrKeyword, keyword::STRICT_OR_RESERVED_KEYWORDS};

// Imports
use crate::{
	Format,
	Parse,
	Print,
	parser::{Parser, ParserError},
};

/// `IDENTIFIER`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an identifier")]
pub enum Identifier {
	NonKw(NonKeywordIdentifier),
	Raw(!),
}

impl AsRef<crate::ParserStr> for Identifier {
	fn as_ref(&self) -> &crate::ParserStr {
		match *self {
			Self::NonKw(ref non_keyword_identifier) => &non_keyword_identifier.0.1,
			Self::Raw(never) => never,
		}
	}
}

/// `NON_KEYWORD_IDENTIFIER`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct NonKeywordIdentifier(pub IdentifierOrKeyword);

impl Parse for NonKeywordIdentifier {
	type Error = NonKeywordIdentifierError;

	#[coverage(off)]
	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let ident = parser
			.parse::<IdentifierOrKeyword>()
			.map_err(NonKeywordIdentifierError::Ident)?;

		if STRICT_OR_RESERVED_KEYWORDS.contains(&parser.str(&ident.1)) {
			return Err(NonKeywordIdentifierError::StrictOrReserved);
		}

		Ok(Self(ident))
	}
}

#[derive(Debug, crate::parser::ParseError)]
pub enum NonKeywordIdentifierError {
	#[parse_error(transparent)]
	Ident(ParserError<IdentifierOrKeyword>),

	#[parse_error(fmt = "Identifier was a strict or reserved keyword")]
	StrictOrReserved,
}
