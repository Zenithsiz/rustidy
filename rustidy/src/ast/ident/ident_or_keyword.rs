//! Identifier or keyword

// Imports
use crate::{Format, Parse, ParserStr, Print, ast::whitespace::Whitespace};

/// `IDENTIFIER_OR_KEYWORD`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = XidStartOrUnderscore, fmt = "Expected `XID_START` or `_`"))]
#[parse(error(name = SingleUnderscore, fmt = "Found `_`"))]
pub struct IdentifierOrKeyword(
	#[format(whitespace)] pub Whitespace,
	#[parse(try_update_with = Self::parse)]
	#[format(str)]
	pub ParserStr,
);

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
