//! Identifier or keyword

// Imports
use crate::{AstStr, Format, Parse, ParseError, Parser, Print, ast::whitespace::Whitespace, parser::ParserError};

/// `IDENTIFIER_OR_KEYWORD`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct IdentOrKeyword(#[format(str)] pub AstStr, #[format(whitespace)] pub Whitespace);

#[derive(Debug, ParseError)]
pub enum IdentOrKeywordError {
	#[parse_error(fmt = "Expected `XID_START`")]
	XidStart,

	#[parse_error(fmt = "Found `_`")]
	SingleUnderscore,

	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),
}

impl Parse for IdentOrKeyword {
	type Error = IdentOrKeywordError;

	#[coverage(off)]
	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let ident = parser.try_update_with(|s| {
			*s = s
				.strip_prefix(unicode_ident::is_xid_start)
				.or_else(|| s.strip_prefix('_'))
				.ok_or(IdentOrKeywordError::XidStart)?;
			*s = s.trim_start_matches(unicode_ident::is_xid_continue);

			Ok(())
		})?;
		if ident.as_str(parser) == "_" {
			return Err(IdentOrKeywordError::SingleUnderscore);
		}
		let whitespace = parser.parse::<Whitespace>().map_err(IdentOrKeywordError::Whitespace)?;

		Ok(Self(ident, whitespace))
	}
}

impl AsRef<Whitespace> for IdentOrKeyword {
	fn as_ref(&self) -> &Whitespace {
		&self.1
	}
}
