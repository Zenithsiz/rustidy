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
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

// Exports
pub use rustidy_ast_tokens::*;

/// `Token`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
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
#[derive(Parse, Format, Print)]
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
