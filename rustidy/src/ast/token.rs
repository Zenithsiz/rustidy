//! Tokens

// Imports
use {
	super::{
		expr::{IntegerLiteral, StringLiteral, without_block::literal::CharLiteral},
		ident::IdentifierOrKeyword,
		lifetime::LifetimeToken,
		whitespace::Whitespace,
	},
	crate::{
		Format,
		ParserStr,
		parser::{Parse, ParseError, Parser, ParserError},
		print::Print,
	},
};

pub macro decl_tokens(
	$raw:ident;
	$(
		$TokenName:ident = $Token:literal
		$( skip_if_tag $skip_if_tag:literal )?
		$( must_not_follow $must_not_follow:literal )*
		;
	)*
) {
	$(
		#[derive(PartialEq, Eq, Clone, Debug)]
		#[derive(serde::Serialize, serde::Deserialize)]
		#[derive(Format, Print)]
		pub struct $TokenName(
			#[format(whitespace)]
			pub Whitespace,

			#[format(str)]
			pub ParserStr,
		);

		#[derive(Debug, ParseError)]
		pub enum ${concat($TokenName, RawError)} {
			#[parse_error(transparent)]
			Whitespace(ParserError<Whitespace>),

			#[parse_error(fmt = concat!("Expected `", $Token, "`"))]
			NotFound,

			#[parse_error(fmt = "Token ends with `XID_CONTINUE` and follows `XID_START` or `_`")]
			FollowsXid,

			$(
				#[parse_error(fmt = concat!("Tag `", $skip_if_tag, "` was present"))]
				Tag,
			)?
		}

		impl Parse for $TokenName {
			type Error = ${concat($TokenName, RawError)};

			#[coverage(off)]
			fn name() -> Option<impl std::fmt::Display> {
				None::<!>
			}

			fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
				$(
					if parser.has_tag($skip_if_tag) {
						return Err(Self::Error::Tag);
					}
				)?

				let ws = parser.parse::<Whitespace>().map_err(Self::Error::Whitespace)?;
				let token = parser.strip_prefix($Token).ok_or(Self::Error::NotFound)?;

				// Note: This checks prevents matching `match` on `matches`
				{
					let token = parser.str(token);
					let remaining = parser.remaining();

					if token.ends_with(unicode_ident::is_xid_continue) &&
						remaining.starts_with(|ch: char| unicode_ident::is_xid_start(ch) || matches!(ch, '_')) {
						return Err(Self::Error::FollowsXid);
					}

				}

				$(
					// TODO: Different error message?
					if parser.strip_prefix($must_not_follow).is_some() {
						return Err(Self::Error::NotFound);
					}
				)*

				Ok(Self(ws, token.into()))
			}
		}
	)*
}

decl_tokens! {
	raw;

	InnerLineDoc = "//!";
	OuterLineDoc = "///";

	Super = "super";
	SelfLower = "self";
	SelfUpper = "Self";
	Crate = "crate" skip_if_tag "skip:`crate`";
	DollarCrate = "$crate";

	As = "as";
	Async = "async";
	Attr = "attr";
	Auto = "auto";
	Await = "await";
	Break = "break";
	Const = "const";
	Continue = "continue";
	Derive = "derive";
	Do = "do";
	Dyn = "dyn";
	Else = "else";
	Enum = "enum";
	Extern = "extern";
	False = "false";
	Fn = "fn";
	For = "for";
	If = "if";
	Impl = "impl";
	In = "in";
	Let = "let";
	Loop = "loop";
	Macro = "macro";
	MacroRules = "macro_rules";
	Match = "match";
	Mod = "mod";
	Move = "move";
	Mut = "mut";
	Pub = "pub";
	Raw = "raw";
	Ref = "ref";
	Return = "return";
	Safe = "safe";
	Static = "static";
	Struct = "struct";
	Trait = "trait";
	True = "true";
	Try = "try";
	Type = "type";
	Union = "union";
	Unsafe = "unsafe";
	Use = "use";
	Where = "where";
	While = "while";
	Yeet = "yeet";

	// Macro frag spec
	Block = "block";
	Expr = "expr";
	Expr2021 = "expr_2021";
	Ident = "ident";
	Item = "item";
	Lifetime = "lifetime";
	Literal = "literal";
	Meta = "meta";
	Pat = "pat";
	PatParam = "pat_param";
	Path = "path";
	Stmt = "stmt";
	Tt = "tt";
	Ty = "ty";
	Vis = "vis";

	// Punctuation
	Eq = '=' must_not_follow '=' must_not_follow '>';
	Lt = '<' must_not_follow '=';
	Le = "<=";
	EqEq = "==";
	Ne = "!=";
	Ge = ">=";
	Gt = '>' must_not_follow '=';
	AndAnd = "&&";
	OrOr = "||";
	Not = '!' must_not_follow '=';
	Tilde = '~';
	Plus = '+' skip_if_tag "skip:`+`" must_not_follow '=';
	Minus = '-' must_not_follow '=' must_not_follow '>';
	Star = '*' skip_if_tag "skip:`*`" must_not_follow '=';
	Slash = '/' must_not_follow '=';
	Percent = '%' must_not_follow '=';
	Caret = '^' must_not_follow '=';
	And = '&' must_not_follow '&' must_not_follow '=';
	Or = '|' must_not_follow '|' must_not_follow '=';
	Shl = "<<" must_not_follow '=';
	Shr = ">>" must_not_follow '=';
	PlusEq = "+=";
	MinusEq = "-=";
	StarEq = "*=";
	SlashEq = "/=";
	PercentEq = "%=";
	CaretEq = "^=";
	AndEq = "&=";
	OrEq = "|=";
	ShlEq = "<<=";
	ShrEq = ">>=";
	At = '@';
	Dot = '.' must_not_follow '.';
	DotDot = ".." must_not_follow '.' must_not_follow '=';
	DotDotDot = "...";
	DotDotEq = "..=";
	Comma = ',';
	Semi = ';';
	Colon = ':' must_not_follow ':';
	PathSep = "::";
	RArrow = "->";
	LArrow = "<-";
	FatArrow = "=>";
	Pound = '#';
	Dollar = '$' skip_if_tag "skip:`$`";
	Question = '?' skip_if_tag "skip:`?`";
	Underscore = '_';
	Quote = '\'';
	DoubleQuote = '"';

	ParenOpen = '(' skip_if_tag "skip:Delimiters";
	ParenClose = ')' skip_if_tag "skip:Delimiters";
	BracketOpen = '[' skip_if_tag "skip:Delimiters";
	BracketClose = ']' skip_if_tag "skip:Delimiters";
	BracesOpen = '{' skip_if_tag "skip:Delimiters";
	BracesClose = '}' skip_if_tag "skip:Delimiters";

}

/// `Token`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum Token {
	IdentOrKeyword(IdentifierOrKeyword),
	RawIdent(!),
	CharLiteral(CharLiteral),
	StringLiteral(StringLiteral),
	RawStringLiteral(!),
	ByteLiteral(!),
	RawByteLiteral(!),
	RawByteStringLiteral(!),
	CStringLiteral(!),
	RawCStringLiteral(!),
	IntegerLiteral(IntegerLiteral),
	FloatLiteral(!),
	LifetimeToken(LifetimeToken),
	Punctuation(Punctuation),
	Reserved(!),
}

/// `Punctuation`
#[derive(PartialEq, Eq, Clone, Debug)]
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
