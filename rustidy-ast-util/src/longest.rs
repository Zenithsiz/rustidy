//! Longest

// TODO: Replace usages of this with peeking at the end of matches.

// Imports
use {
	format::{Format, Formattable},
	parse::{Parse, ParseError, Parser, ParserError},
	print::Print,
};

/// Parses the longest of two types
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumTryAs)]
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
	Both {
		lhs: ParserError<L>,
		rhs: ParserError<R>
	},
}

impl<L: Parse, R: Parse> Parse for Longest<L, R> {
	type Error = LongestError<L, R>;

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let lhs = parser.peek::<L>().map_err(Self::Error::Left)?;
		let rhs = parser.peek::<R>().map_err(Self::Error::Right)?;

		let (value, pos) = match (lhs, rhs) {
			(Ok((lhs, lhs_pos)), Ok((rhs, rhs_pos))) => match lhs_pos > rhs_pos {
				true => (Self::Left(lhs), lhs_pos),
				false => (Self::Right(rhs), rhs_pos),
			},
			(Ok((lhs, pos)), Err(_)) => (Self::Left(lhs), pos),
			(Err(_), Ok((rhs, pos))) => (Self::Right(rhs), pos),
			(Err(lhs), Err(rhs)) => return Err(Self::Error::Both { lhs, rhs }),
		};

		parser.set_pos(pos);
		Ok(value)
	}
}
