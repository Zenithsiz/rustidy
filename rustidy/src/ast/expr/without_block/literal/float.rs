//! Float literal

// Imports
use {
	super::{Suffix, int::DecLiteral},
	crate::{
		Format,
		Parse,
		ParserStr,
		Print,
		ast::{expr::without_block::literal::SuffixNoE, token},
		parser::{Parser, ParserError},
	},
	rustidy_macros::ParseError,
	std::fmt,
};


/// `FLOAT_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct FloatLiteral {
	pub int:      DecLiteral,
	pub dot:      Option<token::Dot>,
	pub frac:     Option<DecLiteral>,
	pub exponent: Option<FloatExponent>,
	pub suffix:   Option<Suffix>,
}

impl Parse for FloatLiteral {
	type Error = FloatLiteralError;

	#[coverage(off)]
	fn name() -> Option<impl fmt::Display> {
		Some("a floating point literal")
	}

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let int = parser.parse::<DecLiteral>()?;

		let (dot, frac) = match parser.try_parse::<token::Dot>()? {
			Ok(dot) => match parser.try_parse::<DecLiteral>()? {
				Ok(frac) => (Some(dot), Some(frac)),
				Err(_) => match parser
					.remaining()
					.starts_with(|ch| matches!(ch, '.' | '_') || unicode_ident::is_xid_start(ch))
				{
					true => return Err(Self::Error::FractionalPartMissing),
					false => (Some(dot), None),
				},
			},
			Err(_) => (None, None),
		};

		let (exponent, suffix) = match (dot.is_some(), frac.is_some()) {
			(true, true) => match parser.try_parse::<FloatExponent>()? {
				Ok(exponent) => (Some(exponent), parser.try_parse::<Suffix>()?.ok()),
				Err(_) => (None, parser.try_parse::<SuffixNoE>()?.ok().map(|suffix| suffix.0)),
			},
			(true, false) => (None, None),
			(false, true) => unreachable!(),
			(false, false) => {
				let exponent = parser.parse::<FloatExponent>()?;
				let suffix = parser.try_parse::<Suffix>()?.ok();
				(Some(exponent), suffix)
			},
		};

		Ok(Self {
			int,
			dot,
			frac,
			exponent,
			suffix,
		})
	}
}

#[derive(Debug, derive_more::From, ParseError)]
pub enum FloatLiteralError {
	#[parse_error(transparent)]
	DecLiteral(ParserError<DecLiteral>),
	#[parse_error(transparent)]
	Dot(ParserError<token::Dot>),
	#[parse_error(transparent)]
	Suffix(ParserError<Suffix>),
	#[parse_error(transparent)]
	SuffixNoE(ParserError<SuffixNoE>),
	#[parse_error(transparent)]
	Exponent(ParserError<FloatExponent>),

	#[parse_error(fmt = "Expected fractional part")]
	FractionalPartMissing,
}

/// `FLOAT_EXPONENT`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct FloatExponent(#[format(str)] pub ParserStr);

impl Parse for FloatExponent {
	type Error = FloatExponentError;

	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let exponent = parser.try_update_with(|s| {
			if !s.starts_with(['e', 'E']) {
				return Err(Self::Error::E);
			}
			*s = &s[1..];

			if s.starts_with(['+', '-']) {
				*s = &s[1..];
			}

			*s = s.trim_start_matches('_');
			*s = s
				.strip_prefix(|ch: char| ch.is_ascii_digit())
				.ok_or(Self::Error::Digit)?;
			*s = s.trim_start_matches(|ch: char| ch.is_ascii_digit() || ch == '_');

			Ok(())
		})?;

		Ok(Self(exponent))
	}
}

#[derive(Debug, derive_more::From, ParseError)]
pub enum FloatExponentError {
	#[parse_error(fmt = "Expected `e` or `E`")]
	E,

	#[parse_error(fmt = "Expected a digit")]
	Digit,
}
