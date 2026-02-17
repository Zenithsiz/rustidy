//! Longest

// TODO: Replace usages of this with peeking at the end of matches.

// Imports
use {
	rustidy_format::{Format, Formattable},
	rustidy_parse::{Parse, ParseError, Parser, ParserError},
	rustidy_print::Print,
};

/// Parses the longest of two types
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Formattable, Format, Print)]
pub enum Longest<L, R> {
	Left(L),
	Right(R),
}

#[derive(derive_more::Debug, ParseError)]
pub enum LongestError<L: Parse, R: Parse> {
	#[parse_error(transparent)]
	Left(ParserError<L>),

	#[parse_error(transparent)]
	Right(ParserError<R>),

	#[parse_error(fmt = "No matches")]
	#[parse_error(multiple)]
	Both { lhs: ParserError<L>, rhs: ParserError<R> },
}

impl<L: Parse, R: Parse> Parse for Longest<L, R> {
	type Error = LongestError<L, R>;

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let lhs = parser.peek::<L>().map_err(Self::Error::Left)?;
		let rhs = parser.peek::<R>().map_err(Self::Error::Right)?;

		let (value, state) = match (lhs, rhs) {
			(Ok((lhs, lhs_state)), Ok((rhs, rhs_state))) => match lhs_state.ahead_of(&rhs_state) {
				true => (Self::Left(lhs), lhs_state),
				false => (Self::Right(rhs), rhs_state),
			},
			(Ok((lhs, pos)), Err(_)) => (Self::Left(lhs), pos),
			(Err(_), Ok((rhs, pos))) => (Self::Right(rhs), pos),
			(Err(lhs), Err(rhs)) => return Err(Self::Error::Both { lhs, rhs }),
		};

		parser.set_peeked(state);
		Ok(value)
	}
}
