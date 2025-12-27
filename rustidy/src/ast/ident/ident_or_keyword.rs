//! Identifier or keyword

// Imports
use crate::{AstStr, Format, Parse, ParseError, Parser, Print, ast::whitespace::Whitespace, parser::ParserError};

/// `IDENTIFIER_OR_KEYWORD` (with whitespace)
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IdentifierOrKeyword(#[format(whitespace)] pub Whitespace, pub IdentifierOrKeywordRaw);


impl AsRef<Whitespace> for IdentifierOrKeyword {
	fn as_ref(&self) -> &Whitespace {
		&self.0
	}
}

/// `IDENTIFIER_OR_KEYWORD` (without whitespace)
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct IdentifierOrKeywordRaw(#[format(str)] pub AstStr);

#[derive(Debug, ParseError)]
pub enum IdentOrKeywordRawError {
	#[parse_error(fmt = "Expected `XID_START`")]
	XidStart,

	#[parse_error(fmt = "Found `_`")]
	SingleUnderscore,

	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),
}

impl Parse for IdentifierOrKeywordRaw {
	type Error = IdentOrKeywordRawError;

	#[coverage(off)]
	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let ident = parser.try_update_with(|s| {
			*s = s
				.strip_prefix(unicode_ident::is_xid_start)
				.or_else(|| s.strip_prefix('_'))
				.ok_or(IdentOrKeywordRawError::XidStart)?;
			*s = s.trim_start_matches(unicode_ident::is_xid_continue);

			Ok(())
		})?;
		if ident.as_str(parser) == "_" {
			return Err(IdentOrKeywordRawError::SingleUnderscore);
		}

		Ok(Self(ident))
	}
}
