//! Lifetime

// Imports
use {
	crate::{IdentifierOrKeyword, NonKeywordIdentifier},
	format::{Format, Formattable, WhitespaceFormat},
	parse::{Parse, Parser, ParserTag},
	print::Print,
	util::Whitespace,
};

/// `Lifetime`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Lifetime(LifetimeToken);

/// `LIFETIME_TOKEN`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum LifetimeToken {
	IdentOrKeyword(QuoteNotQuote<IdentifierOrKeyword>),
	Underscore(QuoteNotQuote<ast_token::Underscore>),
	// TODO: `r#'ident`
}

/// `LIFETIME_OR_LABEL`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum LifetimeOrLabel {
	IdentOrKeyword(QuoteNotQuote<NonKeywordIdentifier>),
	Underscore(QuoteNotQuote<ast_token::Underscore>),
	// TODO: `r#'ident`
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a lifetime token")]
#[parse(error(name = SuffixQuote, fmt = "Unexpected `'`"))]
#[parse(and_try_with = Self::check_suffix_quote)]
pub struct QuoteNotQuote<T> {
	pub quote: ast_token::Quote,
	#[parse(with_tag = ParserTag::SkipWhitespace)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub value: T,
}

impl<T: Parse> QuoteNotQuote<T> {
	pub fn check_suffix_quote(&mut self, parser: &mut Parser) -> Result<(), QuoteNotQuoteError<T>> {
		// If we parse a `'` right after the value, then this is actually a character literal
		// and so we reject it.
		if parser
			.with_tag(
				ParserTag::SkipWhitespace,
				Parser::try_parse::<ast_token::Quote>
			)
			.map_err(QuoteNotQuoteError::Quote)?
			.is_ok() {
			return Err(QuoteNotQuoteError::SuffixQuote);
		}

		Ok(())
	}
}
