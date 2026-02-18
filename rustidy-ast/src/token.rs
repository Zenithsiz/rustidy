//! Tokens

// Imports
use {
	super::lifetime::LifetimeToken,
	rustidy_ast_literal::{
		ByteLiteral,
		ByteStringLiteral,
		CStringLiteral,
		CharLiteral,
		FloatLiteral,
		IntegerLiteral,
		RawByteStringLiteral,
		RawCStringLiteral,
		RawStringLiteral,
		StringLiteral,
	},
	rustidy_ast_util::{IdentifierOrKeyword, RawIdentifier},
	rustidy_format::{Format, Formattable},
	rustidy_parse::{Parse, ParseError},
	rustidy_print::Print,
};

// Exports
pub use rustidy_ast_tokens::*;

/// `Token`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum Token {
	RawIdent(RawIdentifier),
	CharLiteral(CharLiteral),
	StringLiteral(StringLiteral),
	RawStringLiteral(RawStringLiteral),
	ByteLiteral(ByteLiteral),
	ByteStringLiteral(ByteStringLiteral),
	RawByteStringLiteral(RawByteStringLiteral),
	CStringLiteral(CStringLiteral),
	RawCStringLiteral(RawCStringLiteral),
	IntegerLiteral(IntegerLiteral),
	FloatLiteral(FloatLiteral),
	LifetimeToken(LifetimeToken),
	Punctuation(Punctuation),

	IdentOrKeyword(IdentifierOrKeyword),
	// TODO: Reserved tokens? Should we care as a formatter?
}

/// `Punctuation`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Formattable, Format, Print)]
pub enum Punctuation {
	Eq(Eq),
	Lt(Lt),
	Le(Le),
	EqEq(EqEq),
	PunctEquals(Ne),
	Ge(Ge),
	Gt(Gt),
	AndAnd(AndAnd),
	OrOr(OrOr),
	Not(Not),
	Tilde(Tilde),
	Plus(Plus),
	Minus(Minus),
	Star(Star),
	Slash(Slash),
	Percent(Percent),
	Caret(Caret),
	And(And),
	Or(Or),
	Shl(Shl),
	Shr(Shr),
	PlusEq(PlusEq),
	MinusEq(MinusEq),
	StarEq(StarEq),
	SlashEq(SlashEq),
	PercentEq(PercentEq),
	CaretEq(CaretEq),
	AndEq(AndEq),
	OrEq(OrEq),
	ShlEq(ShlEq),
	ShrEq(ShrEq),
	At(At),
	Dot(Dot),
	DotDot(DotDot),
	DotDotDot(DotDotDot),
	DotDotEq(DotDotEq),
	Comma(Comma),
	Semi(Semi),
	Colon(Colon),
	PathSep(PathSep),
	RArrow(RArrow),
	LArrow(LArrow),
	FatArrow(FatArrow),
	Pound(Pound),
	Dollar(Dollar),
	Question(Question),
	Underscore(Underscore),
	ParenOpen(ParenOpen),
	ParenClose(ParenClose),
	BracketOpen(BracketOpen),
	BracketClose(BracketClose),
	BracesOpen(BracesOpen),
	BracesClose(BracesClose),
}

impl rustidy_parse::Parse for Punctuation {
	type Error = PunctuationError;

	fn parse_from(parser: &mut rustidy_parse::Parser) -> Result<Self, Self::Error> {
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Eq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Lt(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Le(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::EqEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::PunctEquals(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Ge(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Gt(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::AndAnd(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::OrOr(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Not(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Tilde(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Plus(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Minus(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Star(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Slash(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Percent(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Caret(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::And(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Or(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Shl(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Shr(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::PlusEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::MinusEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::StarEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::SlashEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::PercentEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::CaretEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::AndEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::OrEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::ShlEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::ShrEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::At(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Dot(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::DotDot(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::DotDotDot(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::DotDotEq(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Comma(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Semi(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Colon(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::PathSep(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::RArrow(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::LArrow(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::FatArrow(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Pound(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Dollar(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Question(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::Underscore(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::ParenOpen(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::ParenClose(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::BracketOpen(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::BracketClose(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::BracesOpen(value));
		}
		if let Ok(value) = parser.try_parse().map_err(|_| PunctuationError)? {
			return Ok(Self::BracesClose(value));
		}

		Err(PunctuationError)
	}
}

#[derive(derive_more::Debug, ParseError)]
#[parse_error(fmt = "Expected punctuation")]
pub struct PunctuationError;
