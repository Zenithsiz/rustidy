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
	rustidy_parse::{Parse, ParseError, ParserError, ParserTag},
	rustidy_print::Print,
	rustidy_util::{StrPopFirst, Whitespace},
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
	Ne(Ne),
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

// TODO: Autogenerate this impl?
impl rustidy_parse::Parse for Punctuation {
	type Error = PunctuationError;

	fn parse_from(parser: &mut rustidy_parse::Parser) -> Result<Self, Self::Error> {
		// TODO: Autogenerate this from `Punctuation`.
		#[derive(Clone, Copy, Debug)]
		enum Punct {
			Eq,
			Lt,
			Le,
			EqEq,
			Ne,
			Ge,
			Gt,
			AndAnd,
			OrOr,
			Not,
			Tilde,
			Plus,
			Minus,
			Star,
			Slash,
			Percent,
			Caret,
			And,
			Or,
			Shl,
			Shr,
			PlusEq,
			MinusEq,
			StarEq,
			SlashEq,
			PercentEq,
			CaretEq,
			AndEq,
			OrEq,
			ShlEq,
			ShrEq,
			At,
			Dot,
			DotDot,
			DotDotDot,
			DotDotEq,
			Comma,
			Semi,
			Colon,
			PathSep,
			RArrow,
			LArrow,
			FatArrow,
			Pound,
			Dollar,
			Question,
			Underscore,
			ParenOpen,
			ParenClose,
			BracketOpen,
			BracketClose,
			BracesOpen,
			BracesClose,
		}

		let skip_plus = parser.has_tag(ParserTag::SkipTokenPlus);
		let skip_star = parser.has_tag(ParserTag::SkipTokenStar);
		let skip_dollar = parser.has_tag(ParserTag::SkipTokenDollar);
		let skip_question = parser.has_tag(ParserTag::SkipTokenQuestion);
		let skip_delimiters = parser.has_tag(ParserTag::SkipDelimiters);

		let ws = parser.parse()?;
		let res = parser
			.try_update_with(|s| {
				let original_s = *s;
				macro punct(
					$len:literal, $punct:ident
				) {
					{
				*s = &original_s[$len..];
				Some(Punct::$punct)
			}
				}
				match s.pop_first()? {
					'=' => match s.pop_first() {
						Some('=') => punct!(2, EqEq),
						Some('>') => punct!(2, FatArrow),
						_ => punct!(1, Eq),
					},
					'<' => match s.pop_first() {
						Some('=') => punct!(2, Le),
						Some('-') => punct!(2, LArrow),
						Some('<') => match s.pop_first() {
							Some('=') => punct!(3, ShlEq),
							_ => punct!(2, Shl),
						},
						_ => punct!(1, Lt),
					},
					'>' => match s.pop_first() {
						Some('=') => punct!(2, Ge),
						Some('>') => match s.pop_first() {
							Some('=') => punct!(3, ShrEq),
							_ => punct!(2, Shr),
						},
						_ => punct!(1, Gt),
					},
					'!' => match s.pop_first() {
						Some('=') => punct!(2, Ne),
						_ => punct!(1, Not),
					},
					'&' => match s.pop_first() {
						Some('&') => punct!(2, AndAnd),
						Some('=') => punct!(2, AndEq),
						_ => punct!(1, And),
					},
					'|' => match s.pop_first() {
						Some('|') => punct!(2, OrOr),
						Some('=') => punct!(2, OrEq),
						_ => punct!(1, Or),
					},
					'+' => match s.pop_first() {
						Some('=') => punct!(2, PlusEq),
						_ if !skip_plus => punct!(1, Plus),
						_ => None,
					},
					'-' => match s.pop_first() {
						Some('=') => punct!(2, MinusEq),
						Some('>') => punct!(2, RArrow),
						_ => punct!(1, Minus),
					},
					'*' => match s.pop_first() {
						Some('=') => punct!(2, StarEq),
						_ if !skip_star => punct!(1, Star),
						_ => None,
					},
					'/' => match s.pop_first() {
						Some('=') => punct!(2, SlashEq),
						_ => punct!(1, Slash),
					},
					'^' => match s.pop_first() {
						Some('=') => punct!(2, CaretEq),
						_ => punct!(1, Caret),
					},
					'%' => match s.pop_first() {
						Some('=') => punct!(2, PercentEq),
						_ => punct!(1, Percent),
					},
					'.' => match s.pop_first() {
						Some('.') => match s.pop_first() {
							Some('.') => punct!(3, DotDotDot),
							Some('=') => punct!(3, DotDotEq),
							_ => punct!(2, DotDot),
						},
						_ => punct!(1, Dot),
					},
					':' => match s.pop_first() {
						Some(':') => punct!(2, PathSep),
						_ => punct!(1, Colon),
					},
					'~' => punct!(1, Tilde),
					'@' => punct!(1, At),
					',' => punct!(1, Comma),
					';' => punct!(1, Semi),
					'#' => punct!(1, Pound),
					'$' if !skip_dollar => punct!(1, Dollar),
					'?' if !skip_question => punct!(1, Question),
					'_' => punct!(1, Underscore),
					'(' if !skip_delimiters => punct!(1, ParenOpen),
					')' if !skip_delimiters => punct!(1, ParenClose),
					'[' if !skip_delimiters => punct!(1, BracketOpen),
					']' if !skip_delimiters => punct!(1, BracketClose),
					'{' if !skip_delimiters => punct!(1, BracesOpen),
					'}' if !skip_delimiters => punct!(1, BracesClose),
					_ => None,
				}
			});
		let (s, punct) = res.ok_or(PunctuationError::NotFound)?;

		let punct = match punct {
			Punct::Eq => Self::Eq(Eq(ws, s)),
			Punct::Lt => Self::Lt(Lt(ws, s)),
			Punct::Le => Self::Le(Le(ws, s)),
			Punct::EqEq => Self::EqEq(EqEq(ws, s)),
			Punct::Ne => Self::Ne(Ne(ws, s)),
			Punct::Ge => Self::Ge(Ge(ws, s)),
			Punct::Gt => Self::Gt(Gt(ws, s)),
			Punct::AndAnd => Self::AndAnd(AndAnd(ws, s)),
			Punct::OrOr => Self::OrOr(OrOr(ws, s)),
			Punct::Not => Self::Not(Not(ws, s)),
			Punct::Tilde => Self::Tilde(Tilde(ws, s)),
			Punct::Plus => Self::Plus(Plus(ws, s)),
			Punct::Minus => Self::Minus(Minus(ws, s)),
			Punct::Star => Self::Star(Star(ws, s)),
			Punct::Slash => Self::Slash(Slash(ws, s)),
			Punct::Percent => Self::Percent(Percent(ws, s)),
			Punct::Caret => Self::Caret(Caret(ws, s)),
			Punct::And => Self::And(And(ws, s)),
			Punct::Or => Self::Or(Or(ws, s)),
			Punct::Shl => Self::Shl(Shl(ws, s)),
			Punct::Shr => Self::Shr(Shr(ws, s)),
			Punct::PlusEq => Self::PlusEq(PlusEq(ws, s)),
			Punct::MinusEq => Self::MinusEq(MinusEq(ws, s)),
			Punct::StarEq => Self::StarEq(StarEq(ws, s)),
			Punct::SlashEq => Self::SlashEq(SlashEq(ws, s)),
			Punct::PercentEq => Self::PercentEq(PercentEq(ws, s)),
			Punct::CaretEq => Self::CaretEq(CaretEq(ws, s)),
			Punct::AndEq => Self::AndEq(AndEq(ws, s)),
			Punct::OrEq => Self::OrEq(OrEq(ws, s)),
			Punct::ShlEq => Self::ShlEq(ShlEq(ws, s)),
			Punct::ShrEq => Self::ShrEq(ShrEq(ws, s)),
			Punct::At => Self::At(At(ws, s)),
			Punct::Dot => Self::Dot(Dot(ws, s)),
			Punct::DotDot => Self::DotDot(DotDot(ws, s)),
			Punct::DotDotDot => Self::DotDotDot(DotDotDot(ws, s)),
			Punct::DotDotEq => Self::DotDotEq(DotDotEq(ws, s)),
			Punct::Comma => Self::Comma(Comma(ws, s)),
			Punct::Semi => Self::Semi(Semi(ws, s)),
			Punct::Colon => Self::Colon(Colon(ws, s)),
			Punct::PathSep => Self::PathSep(PathSep(ws, s)),
			Punct::RArrow => Self::RArrow(RArrow(ws, s)),
			Punct::LArrow => Self::LArrow(LArrow(ws, s)),
			Punct::FatArrow => Self::FatArrow(FatArrow(ws, s)),
			Punct::Pound => Self::Pound(Pound(ws, s)),
			Punct::Dollar => Self::Dollar(Dollar(ws, s)),
			Punct::Question => Self::Question(Question(ws, s)),
			Punct::Underscore => Self::Underscore(Underscore(ws, s)),
			Punct::ParenOpen => Self::ParenOpen(ParenOpen(ws, s)),
			Punct::ParenClose => Self::ParenClose(ParenClose(ws, s)),
			Punct::BracketOpen => Self::BracketOpen(BracketOpen(ws, s)),
			Punct::BracketClose => Self::BracketClose(BracketClose(ws, s)),
			Punct::BracesOpen => Self::BracesOpen(BracesOpen(ws, s)),
			Punct::BracesClose => Self::BracesClose(BracesClose(ws, s)),
		};

		Ok(punct)
	}
}

#[derive(derive_more::Debug, derive_more::From, ParseError)]
pub enum PunctuationError {
	#[parse_error(transparent)]
	Whitespace(ParserError<Whitespace>),

	#[parse_error(fmt = "Expected punctuation")]
	NotFound,
}
