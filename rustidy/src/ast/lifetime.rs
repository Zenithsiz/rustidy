//! Lifetime

// Imports
use {
	super::{
		ident::{IdentifierOrKeyword, NonKeywordIdentifier},
		token,
	},
	crate::{Format, Parse, ParseError, Parser, Print, parser::ParserError},
	std::fmt,
};

/// `Lifetime`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Lifetime(LifetimeToken);

/// `LIFETIME_TOKEN`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LifetimeToken {
	IdentOrKeyword(QuoteNotQuote<IdentifierOrKeyword>),
	Underscore(QuoteNotQuote<token::Underscore>),
	// TODO: `r#'ident`
}

/// `LIFETIME_OR_LABEL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LifetimeOrLabel {
	IdentOrKeyword(QuoteNotQuote<NonKeywordIdentifier>),
	Underscore(QuoteNotQuote<token::Underscore>),
	// TODO: `r#'ident`
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct QuoteNotQuote<T> {
	pub quote: token::Quote,
	pub value: T,
}

#[derive(derive_more::Debug, ParseError)]
pub enum LifetimeIdentOrKeywordError<T: Parse> {
	#[parse_error(transparent)]
	Quote(ParserError<token::Quote>),

	#[parse_error(transparent)]
	Name(ParserError<T>),

	#[parse_error(fmt = "Unexpected `'`")]
	SuffixQuote,
}

impl<T: Parse> Parse for QuoteNotQuote<T> {
	type Error = LifetimeIdentOrKeywordError<T>;

	fn name() -> Option<impl fmt::Display> {
		Some("a lifetime token")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let quote = parser.parse().map_err(Self::Error::Quote)?;
		let value = parser
			.with_tag("skip:Whitespace", Parser::parse::<T>)
			.map_err(Self::Error::Name)?;

		// If we parse a `'` right after the value, then this is actually a character literal
		// and so we reject it.
		if parser.try_parse::<token::Quote>().map_err(Self::Error::Quote)?.is_ok() {
			return Err(Self::Error::SuffixQuote);
		}

		Ok(Self { quote, value })
	}
}
