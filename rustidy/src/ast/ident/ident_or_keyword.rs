//! Identifier or keyword

// Imports
use crate::{Format, Parse, ParseError, Parser, ParserStr, Print, ast::whitespace::Whitespace, parser::ParserError};

/// `IDENTIFIER_OR_KEYWORD`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct IdentifierOrKeyword(#[format(whitespace)] pub Whitespace, #[format(str)] pub ParserStr);

#[derive(Debug, ParseError)]
pub enum IdentOrKeywordError {
	#[parse_error(fmt = "Expected `XID_START`")]
	XidStart,

	#[parse_error(fmt = "Found `_`")]
	SingleUnderscore,

	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),
}

impl Parse for IdentifierOrKeyword {
	type Error = IdentOrKeywordError;

	#[coverage(off)]
	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let ws = parser.parse::<Whitespace>().map_err(IdentOrKeywordError::Whitespace)?;
		let ident = parser.try_update_with(|s| {
			*s = s
				.strip_prefix(unicode_ident::is_xid_start)
				.or_else(|| s.strip_prefix('_'))
				.ok_or(IdentOrKeywordError::XidStart)?;
			*s = s.trim_start_matches(unicode_ident::is_xid_continue);

			Ok(())
		})?;
		if parser.str(ident) == "_" {
			return Err(IdentOrKeywordError::SingleUnderscore);
		}

		Ok(Self(ws, ident))
	}
}
