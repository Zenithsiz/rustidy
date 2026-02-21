//! Identifier

// Modules
pub mod ident_or_keyword;

// Exports
pub use self::ident_or_keyword::{IdentifierOrKeyword, RawIdentifier};

// Imports
use {
	rustidy_format::{Format, Formattable},
	rustidy_parse::{Parse, Parser},
	rustidy_print::Print,
	std::borrow::Cow,
};

/// `IDENTIFIER`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "an identifier")]
pub enum Identifier {
	Raw(RawIdentifier),
	NonKw(NonKeywordIdentifier),
}

impl Identifier {
	/// Returns this path as a string.
	#[must_use]
	pub fn as_str<'a>(&'a self, input: &'a str) -> Cow<'a, str> {
		match self {
			// TODO: How should we handle raw identifiers?
			Self::Raw(_) => todo!("Raw identifiers aren't fully implemented"),
			Self::NonKw(ident) => ident.0.1.str(input),
		}
	}

	/// Returns if this identifier is the same as `ident`.
	///
	/// For raw identifiers, the `r#` prefix isn't included in
	/// the comparison, so `r#abc` would return true for `"abc"`.
	#[must_use]
	pub fn is_str(&self, input: &str, ident: &str) -> bool {
		match self {
			Self::Raw(_) => todo!("Raw identifiers aren't fully implemented"),
			Self::NonKw(this) => this.0.1.is_str(input, ident),
		}
	}

	/// Returns the identifier length not including whitespace
	#[must_use]
	pub fn non_ws_len(&self) -> usize {
		match self {
			Self::Raw(ident) => ident.1.len(),
			Self::NonKw(ident) => ident.0.1.len(),
		}
	}
}

/// `NON_KEYWORD_IDENTIFIER`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = StrictOrReserved, fmt = "Identifier was a strict or reserved keyword"))]
#[parse(and_try_with = Self::check_strict_reserved)]
pub struct NonKeywordIdentifier(pub IdentifierOrKeyword);

impl NonKeywordIdentifier {
	pub fn check_strict_reserved(&mut self, parser: &mut Parser) -> Result<(), NonKeywordIdentifierError> {
		if STRICT_OR_RESERVED_KEYWORDS
			.contains(&&*parser.str(&self.0.1)) {
			return Err(NonKeywordIdentifierError::StrictOrReserved);
		}

		Ok(())
	}
}

/// Strict/Reserved keywords
#[rustfmt::skip]
pub const STRICT_OR_RESERVED_KEYWORDS: [&str; 52] = [
	// Strict (2015)
	"as",
	"break",
	"const",
	"continue",
	"crate",
	"else",
	"enum",
	"extern",
	"false",
	"fn",
	"for",
	"if",
	"impl",
	"in",
	"let",
	"loop",
	"match",
	"mod",
	"move",
	"mut",
	"pub",
	"ref",
	"return",
	"self",
	"Self",
	"static",
	"struct",
	"super",
	"trait",
	"true",
	"type",
	"unsafe",
	"use",
	"where",
	"while",
	// Strict (2015)
	"async",
	"await",
	"dyn",
	// Reserved (2015)
	"abstract",
	"become",
	"box",
	"do",
	"final",
	"macro",
	"override",
	"priv",
	"typeof",
	"unsized",
	"virtual",
	"yield",
	// Reserved (2018)
	"try",
	// Reserved (2024)
	"try",
];
