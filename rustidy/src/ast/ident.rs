//! Identifier

// Modules
pub mod ident_or_keyword;
pub mod keyword;

// Exports
pub use self::{
	ident_or_keyword::{IdentifierOrKeyword, IdentifierOrKeywordRaw},
	keyword::STRICT_OR_RESERVED_KEYWORDS,
};

// Imports
use {
	super::whitespace::Whitespace,
	crate::{
		Format,
		Parse,
		Print,
		parser::{Parser, ParserError},
	},
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

impl AsRef<crate::AstStr> for Identifier {
	fn as_ref(&self) -> &crate::AstStr {
		match *self {
			Self::NonKw(ref non_keyword_identifier) => &non_keyword_identifier.0.1.0,
			Self::Raw(never) => never,
		}
	}
}

/// `NON_KEYWORD_IDENTIFIER` (with whitespace)
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

		if STRICT_OR_RESERVED_KEYWORDS.contains(&parser.str(&ident.1.0)) {
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

impl AsRef<Whitespace> for NonKeywordIdentifier {
	fn as_ref(&self) -> &Whitespace {
		self.0.as_ref()
	}
}

/// `NON_KEYWORD_IDENTIFIER` (without whitespace)
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct NonKeywordIdentifierRaw(pub IdentifierOrKeywordRaw);

impl Parse for NonKeywordIdentifierRaw {
	type Error = NonKeywordIdentifierRawError;

	#[coverage(off)]
	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let ident = parser
			.parse::<IdentifierOrKeywordRaw>()
			.map_err(NonKeywordIdentifierRawError::Ident)?;

		if STRICT_OR_RESERVED_KEYWORDS.contains(&parser.str(&ident.0)) {
			return Err(NonKeywordIdentifierRawError::StrictOrReserved);
		}

		Ok(Self(ident))
	}
}

#[derive(Debug, crate::parser::ParseError)]
pub enum NonKeywordIdentifierRawError {
	#[parse_error(transparent)]
	Ident(ParserError<IdentifierOrKeywordRaw>),

	#[parse_error(fmt = "Identifier was a strict or reserved keyword")]
	StrictOrReserved,
}
