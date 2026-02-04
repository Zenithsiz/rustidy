//! Follows

// Imports
use rustidy_parse::{Parse, ParserError};

/// Follows.
///
/// Parses a type without advancing the parser.
pub struct Follows<T>(pub T);

impl<T: Parse> Parse for Follows<T> {
	type Error = ParserError<T>;

	#[coverage(on)]
	fn parse_from(parser: &mut rustidy_parse::Parser) -> Result<Self, Self::Error> {
		let (value, _) = parser.peek::<T>()??;
		Ok(Self(value))
	}
}
