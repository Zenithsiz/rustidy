//! Follows

// Imports
use parse::Parse;

/// Follows.
///
/// Parses a type without advancing the parser.
pub struct Follows<T>(pub T);

impl<T: Parse> Parse for Follows<T> {
	type Error = T::Error;

	fn name() -> Option<impl std::fmt::Display> {
		T::name()
	}

	fn parse_from(parser: &mut parse::Parser) -> Result<Self, Self::Error> {
		let (value, _) = parser.peek_with(T::parse_from)??;
		Ok(Self(value))
	}
}
