//! Lifetime

// Imports
use {
	super::{
		ident::{IdentOrKeyword, NonKeywordIdentifier},
		token,
		whitespace::Whitespace,
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
	IdentOrKeyword(QuoteNotQuote<IdentOrKeyword>),
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
	quote: token::Quote,
	value: T,
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

impl<T: Parse + AsRef<Whitespace>> Parse for QuoteNotQuote<T> {
	type Error = LifetimeIdentOrKeywordError<T>;

	fn name() -> Option<impl fmt::Display> {
		Some("a lifetime token")
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		// TODO: Should we allow whitespace *after* the first token?
		let quote = parser.parse().map_err(Self::Error::Quote)?;
		let value = parser.parse().map_err(Self::Error::Name)?;

		// If we have no whitespace, and the next token is a `'`, we reject this lifetime, since
		// it's actually a character literal
		if value.as_ref().is_empty() && parser.try_parse::<token::Quote>().map_err(Self::Error::Quote)?.is_ok() {
			return Err(Self::Error::SuffixQuote);
		}

		Ok(Self { quote, value })
	}
}
