//! Not follows

// Imports
use {core::marker::PhantomData, parse::{Parse, ParseError, ParserError}};

/// Not follows.
///
/// Ensures that a type *cannot* be parsed at the current position
pub struct NotFollows<T>(PhantomData<T>);

impl<T: Parse> Parse for NotFollows<T> {
	type Error = NotFollowsError<T>;

	#[coverage(on)]
	fn parse_from(parser: &mut parse::Parser) -> Result<Self, Self::Error> {
		match parser.peek::<T>()? {
			Ok(_) => Err(NotFollowsError::Parsed),
			Err(_) => Ok(Self(PhantomData)),
		}
	}
}

#[derive(derive_more::Debug, derive_more::From, ParseError)]
pub enum NotFollowsError<T: Parse> {
	#[parse_error(transparent)]
	Inner(ParserError<T>),

	#[parse_error(fmt = "Unexpected value")]
	Parsed,
}
