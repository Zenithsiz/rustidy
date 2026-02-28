//! Tokens

// Imports
use {
	crate::{
		ByteLiteral,
		ByteStringLiteral,
		CStringLiteral,
		CharLiteral,
		FloatLiteral,
		IdentifierOrKeyword,
		IntegerLiteral,
		LifetimeToken,
		RawByteStringLiteral,
		RawCStringLiteral,
		RawIdentifier,
		RawStringLiteral,
		StringLiteral,
	},
	format::{Format, Formattable},
	parse::{Parse, ParseError, ParserError, ParserTag},
	print::Print,
	util::{StrPopFirst, Whitespace},
};

/// `Token`
#[derive(PartialEq, Eq, Clone, Debug)]
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Formattable, Format, Print)]
pub enum Punctuation {
	Eq(ast_token::Eq),
	Lt(ast_token::Lt),
	Le(ast_token::Le),
	EqEq(ast_token::EqEq),
	Ne(ast_token::Ne),
	Ge(ast_token::Ge),
	Gt(ast_token::Gt),
	AndAnd(ast_token::AndAnd),
	OrOr(ast_token::OrOr),
	Not(ast_token::Not),
	Tilde(ast_token::Tilde),
	Plus(ast_token::Plus),
	Minus(ast_token::Minus),
	Star(ast_token::Star),
	Slash(ast_token::Slash),
	Percent(ast_token::Percent),
	Caret(ast_token::Caret),
	And(ast_token::And),
	Or(ast_token::Or),
	Shl(ast_token::Shl),
	Shr(ast_token::Shr),
	PlusEq(ast_token::PlusEq),
	MinusEq(ast_token::MinusEq),
	StarEq(ast_token::StarEq),
	SlashEq(ast_token::SlashEq),
	PercentEq(ast_token::PercentEq),
	CaretEq(ast_token::CaretEq),
	AndEq(ast_token::AndEq),
	OrEq(ast_token::OrEq),
	ShlEq(ast_token::ShlEq),
	ShrEq(ast_token::ShrEq),
	At(ast_token::At),
	Dot(ast_token::Dot),
	DotDot(ast_token::DotDot),
	DotDotDot(ast_token::DotDotDot),
	DotDotEq(ast_token::DotDotEq),
	Comma(ast_token::Comma),
	Semi(ast_token::Semi),
	Colon(ast_token::Colon),
	PathSep(ast_token::PathSep),
	RArrow(ast_token::RArrow),
	LArrow(ast_token::LArrow),
	FatArrow(ast_token::FatArrow),
	Pound(ast_token::Pound),
	Dollar(ast_token::Dollar),
	Question(ast_token::Question),
	Underscore(ast_token::Underscore),
	ParenOpen(ast_token::ParenOpen),
	ParenClose(ast_token::ParenClose),
	BracketOpen(ast_token::BracketOpen),
	BracketClose(ast_token::BracketClose),
	BracesOpen(ast_token::BracesOpen),
	BracesClose(ast_token::BracesClose),
}

// TODO: Autogenerate this impl?
impl parse::Parse for Punctuation {
	type Error = PunctuationError;

	fn parse_from(parser: &mut parse::Parser) -> Result<Self, Self::Error> {
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
		let res = parser.try_update_with(|s| {
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
			Punct::Eq => Self::Eq(ast_token::Eq(ws, s)),
			Punct::Lt => Self::Lt(ast_token::Lt(ws, s)),
			Punct::Le => Self::Le(ast_token::Le(ws, s)),
			Punct::EqEq => Self::EqEq(ast_token::EqEq(ws, s)),
			Punct::Ne => Self::Ne(ast_token::Ne(ws, s)),
			Punct::Ge => Self::Ge(ast_token::Ge(ws, s)),
			Punct::Gt => Self::Gt(ast_token::Gt(ws, s)),
			Punct::AndAnd => Self::AndAnd(ast_token::AndAnd(ws, s)),
			Punct::OrOr => Self::OrOr(ast_token::OrOr(ws, s)),
			Punct::Not => Self::Not(ast_token::Not(ws, s)),
			Punct::Tilde => Self::Tilde(ast_token::Tilde(ws, s)),
			Punct::Plus => Self::Plus(ast_token::Plus(ws, s)),
			Punct::Minus => Self::Minus(ast_token::Minus(ws, s)),
			Punct::Star => Self::Star(ast_token::Star(ws, s)),
			Punct::Slash => Self::Slash(ast_token::Slash(ws, s)),
			Punct::Percent => Self::Percent(ast_token::Percent(ws, s)),
			Punct::Caret => Self::Caret(ast_token::Caret(ws, s)),
			Punct::And => Self::And(ast_token::And(ws, s)),
			Punct::Or => Self::Or(ast_token::Or(ws, s)),
			Punct::Shl => Self::Shl(ast_token::Shl(ws, s)),
			Punct::Shr => Self::Shr(ast_token::Shr(ws, s)),
			Punct::PlusEq => Self::PlusEq(ast_token::PlusEq(ws, s)),
			Punct::MinusEq => Self::MinusEq(ast_token::MinusEq(ws, s)),
			Punct::StarEq => Self::StarEq(ast_token::StarEq(ws, s)),
			Punct::SlashEq => Self::SlashEq(ast_token::SlashEq(ws, s)),
			Punct::PercentEq => Self::PercentEq(ast_token::PercentEq(ws, s)),
			Punct::CaretEq => Self::CaretEq(ast_token::CaretEq(ws, s)),
			Punct::AndEq => Self::AndEq(ast_token::AndEq(ws, s)),
			Punct::OrEq => Self::OrEq(ast_token::OrEq(ws, s)),
			Punct::ShlEq => Self::ShlEq(ast_token::ShlEq(ws, s)),
			Punct::ShrEq => Self::ShrEq(ast_token::ShrEq(ws, s)),
			Punct::At => Self::At(ast_token::At(ws, s)),
			Punct::Dot => Self::Dot(ast_token::Dot(ws, s)),
			Punct::DotDot => Self::DotDot(ast_token::DotDot(ws, s)),
			Punct::DotDotDot => Self::DotDotDot(ast_token::DotDotDot(ws, s)),
			Punct::DotDotEq => Self::DotDotEq(ast_token::DotDotEq(ws, s)),
			Punct::Comma => Self::Comma(ast_token::Comma(ws, s)),
			Punct::Semi => Self::Semi(ast_token::Semi(ws, s)),
			Punct::Colon => Self::Colon(ast_token::Colon(ws, s)),
			Punct::PathSep => Self::PathSep(ast_token::PathSep(ws, s)),
			Punct::RArrow => Self::RArrow(ast_token::RArrow(ws, s)),
			Punct::LArrow => Self::LArrow(ast_token::LArrow(ws, s)),
			Punct::FatArrow => Self::FatArrow(ast_token::FatArrow(ws, s)),
			Punct::Pound => Self::Pound(ast_token::Pound(ws, s)),
			Punct::Dollar => Self::Dollar(ast_token::Dollar(ws, s)),
			Punct::Question => Self::Question(ast_token::Question(ws, s)),
			Punct::Underscore => Self::Underscore(ast_token::Underscore(ws, s)),
			Punct::ParenOpen => Self::ParenOpen(ast_token::ParenOpen(ws, s)),
			Punct::ParenClose => Self::ParenClose(ast_token::ParenClose(ws, s)),
			Punct::BracketOpen => Self::BracketOpen(ast_token::BracketOpen(ws, s)),
			Punct::BracketClose => Self::BracketClose(ast_token::BracketClose(ws, s)),
			Punct::BracesOpen => Self::BracesOpen(ast_token::BracesOpen(ws, s)),
			Punct::BracesClose => Self::BracesClose(ast_token::BracesClose(ws, s)),
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
