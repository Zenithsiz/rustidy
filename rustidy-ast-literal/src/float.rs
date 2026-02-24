//! Float literal

// Imports
use {
	crate::SuffixNoE,
	super::{Suffix, int::DecLiteral},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_macros::ParseError,
	rustidy_parse::{Parse, Parser, ParserError, ParserTag},
	rustidy_print::Print,
	rustidy_util::{AstStr, Whitespace},
	std::fmt,
};


/// `FLOAT_LITERAL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Formattable, Format, Print)]
pub struct FloatLiteral {
	pub ws:       Whitespace,
	#[format(prefix_ws = ())]
	pub int:      DecLiteral,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot:      Option<rustidy_ast_tokens::Dot>,
	#[format(prefix_ws = ())]
	pub frac:     Option<DecLiteral>,
	#[format(prefix_ws = ())]
	pub exponent: Option<FloatExponent>,
	#[format(prefix_ws = ())]
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
		let ws = parser.parse::<Whitespace>()?;
		let int = parser.parse::<DecLiteral>()?;

		let (dot, frac) = match parser.with_tag(
			ParserTag::SkipWhitespace,
			Parser::try_parse::<rustidy_ast_tokens::Dot>
		)? {
			Ok(dot) => match parser.try_parse::<DecLiteral>()? {
				Ok(frac) => (Some(dot), Some(frac)),
				Err(_) => match parser.remaining().starts_with(
					|ch| matches!(ch, '.' | '_') || unicode_ident::is_xid_start(ch)
				) {
					true => return Err(Self::Error::FractionalPartMissing),
					false => (Some(dot), None),
				},
			},
			Err(_) => (None, None),
		};

		let (exponent, suffix) = match (dot.is_some(), frac.is_some()) {
			(true, true) => match parser.try_parse::<FloatExponent>()? {
				Ok(exponent) => (Some(exponent), parser.try_parse::<Suffix>()?.ok()),
				Err(_) => (None, parser
					.try_parse::<SuffixNoE>()?
					.ok()
					.map(|suffix| suffix.0)),
			},
			(true, false) => (None, None),
			(false, true) => unreachable!(),
			(false, false) => {
				let exponent = parser.parse::<FloatExponent>()?;
				let suffix = parser.try_parse::<Suffix>()?.ok();
				(Some(exponent), suffix)
			},
		};

		Ok(Self { ws, int, dot, frac, exponent, suffix, })
	}
}

#[derive(Debug, derive_more::From, ParseError)]
pub enum FloatLiteralError {
	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),
	#[parse_error(transparent)]
	DecLiteral(ParserError<DecLiteral>),
	#[parse_error(transparent)]
	Dot(ParserError<rustidy_ast_tokens::Dot>),
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
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = E, fmt = "Expected `e` or `E`"))]
#[parse(error(name = Digit, fmt = "Expected a digit"))]
#[format(no_prefix_ws)]
pub struct FloatExponent(#[parse(try_update_with = Self::parse)]
#[format(str)]
pub AstStr);

impl FloatExponent {
	fn parse(s: &mut &str) -> Result<(), FloatExponentError> {
		*s = s
			.strip_prefix(['e', 'E'])
			.ok_or(FloatExponentError::E)?;
		*s = s.trim_prefix(['+', '-']);
		*s = s.trim_start_matches('_');
		*s = s
			.strip_prefix(|ch: char| ch.is_ascii_digit())
			.ok_or(FloatExponentError::Digit)?;
		*s = s
			.trim_start_matches(|ch: char| ch.is_ascii_digit() || ch == '_');

		Ok(())
	}
}
