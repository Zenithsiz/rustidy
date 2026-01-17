//! Lifetime

// Imports
use {
	super::{
		ident::{IdentifierOrKeyword, NonKeywordIdentifier},
		token,
	},
	crate::{Format, Parse, Parser, Print},
};

/// `Lifetime`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Lifetime(LifetimeToken);

/// `LIFETIME_TOKEN`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LifetimeToken {
	IdentOrKeyword(QuoteNotQuote<IdentifierOrKeyword>),
	Underscore(QuoteNotQuote<token::Underscore>),
	// TODO: `r#'ident`
}

/// `LIFETIME_OR_LABEL`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum LifetimeOrLabel {
	IdentOrKeyword(QuoteNotQuote<NonKeywordIdentifier>),
	Underscore(QuoteNotQuote<token::Underscore>),
	// TODO: `r#'ident`
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a lifetime token")]
#[parse(error(name = SuffixQuote, fmt = "Unexpected `'`"))]
#[parse(and_try_with = Self::check_suffix_quote)]
pub struct QuoteNotQuote<T> {
	pub quote: token::Quote,
	#[parse(with_tag = "skip:Whitespace")]
	pub value: T,
}

impl<T: Parse> QuoteNotQuote<T> {
	pub fn check_suffix_quote(&mut self, parser: &mut Parser) -> Result<(), QuoteNotQuoteError<T>> {
		// If we parse a `'` right after the value, then this is actually a character literal
		// and so we reject it.
		if parser
			.with_tag("skip:Whitespace", Parser::try_parse::<token::Quote>)
			.map_err(QuoteNotQuoteError::Quote)?
			.is_ok()
		{
			return Err(QuoteNotQuoteError::SuffixQuote);
		}

		Ok(())
	}
}
