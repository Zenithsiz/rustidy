//! Identifier or keyword

// Imports
use crate::{Format, Parse, ParserStr, Print, ast::whitespace::Whitespace, parser};

/// `IDENTIFIER_OR_KEYWORD`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = XidStartOrUnderscore, fmt = "Expected `XID_START` or `_`"))]
#[parse(error(name = SingleUnderscore, fmt = "Found `_`"))]
pub struct IdentifierOrKeyword(pub Whitespace, #[parse(try_update_with = Self::parse)] pub ParserStr);

impl IdentifierOrKeyword {
	fn parse(s: &mut &str) -> Result<(), IdentifierOrKeywordError> {
		match s.strip_prefix(unicode_ident::is_xid_start) {
			Some(rest) => *s = rest,
			None => {
				*s = s
					.strip_prefix('_')
					.ok_or(IdentifierOrKeywordError::XidStartOrUnderscore)?;
				if !s.starts_with(unicode_ident::is_xid_continue) {
					return Err(IdentifierOrKeywordError::SingleUnderscore);
				}
			},
		}
		*s = s.trim_start_matches(unicode_ident::is_xid_continue);

		Ok(())
	}
}

/// `RAW_IDENTIFIER`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Raw, fmt = "Expected `r#`"))]
#[parse(error(name = IdentOrKeyword(IdentifierOrKeywordError), transparent))]
#[parse(error(name = ForbiddenKeyword, fmt = "Raw identifier cannot be `crate`, `self`, `super` or `Self`"))]
pub struct RawIdentifier(pub Whitespace, #[parse(try_update_with = Self::parse)] pub ParserStr);

impl RawIdentifier {
	fn parse(s: &mut &str) -> Result<(), RawIdentifierError> {
		*s = s.strip_prefix("r#").ok_or(RawIdentifierError::Raw)?;
		let ident =
			parser::parse_from_str(s, IdentifierOrKeyword::parse).map_err(RawIdentifierError::IdentOrKeyword)?;
		if ["crate", "self", "super", "Self"].contains(&ident) {
			return Err(RawIdentifierError::ForbiddenKeyword);
		}

		Ok(())
	}
}
