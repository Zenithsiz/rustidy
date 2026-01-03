//! Identifier

// Modules
pub mod ident_or_keyword;
pub mod keyword;

// Exports
pub use self::{ident_or_keyword::IdentifierOrKeyword, keyword::STRICT_OR_RESERVED_KEYWORDS};

// Imports
use {
	self::ident_or_keyword::RawIdentifier,
	crate::{Format, Parse, Print, parser::Parser},
};

/// `IDENTIFIER`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an identifier")]
pub enum Identifier {
	Raw(RawIdentifier),
	NonKw(NonKeywordIdentifier),
}

/// `NON_KEYWORD_IDENTIFIER`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = StrictOrReserved, fmt = "Identifier was a strict or reserved keyword"))]
#[parse(and_try_with = Self::check_strict_reserved)]
pub struct NonKeywordIdentifier(pub IdentifierOrKeyword);

impl NonKeywordIdentifier {
	pub fn check_strict_reserved(&mut self, parser: &mut Parser) -> Result<(), NonKeywordIdentifierError> {
		if STRICT_OR_RESERVED_KEYWORDS.contains(&parser.str(self.0.1)) {
			return Err(NonKeywordIdentifierError::StrictOrReserved);
		}

		Ok(())
	}
}
