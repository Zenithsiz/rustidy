//! Tokens

// Imports
use {
	super::{
		expr::{
			IntegerLiteral,
			StringLiteral,
			without_block::literal::{
				ByteLiteral,
				ByteStringLiteral,
				CStringLiteral,
				CharLiteral,
				FloatLiteral,
				RawByteStringLiteral,
				RawCStringLiteral,
				RawStringLiteral,
			},
		},
		ident::{IdentifierOrKeyword, ident_or_keyword::RawIdentifier},
		lifetime::LifetimeToken,
		whitespace::Whitespace,
	},
	crate::{
		Format,
		ParserStr,
		parser::{Parse, Parser},
		print::Print,
	},
};

pub macro decl_tokens(
	$(
		$TokenName:ident = $Token:literal
		$( skip_if_tag $skip_if_tag:literal )?
		$( must_not_follow $must_not_follow:literal )*
		;
	)*
) {
	$(
		#[derive(PartialEq, Eq, Debug)]
		#[derive(serde::Serialize, serde::Deserialize)]
		#[derive(Parse, Format, Print)]
		#[parse(error(name = NotFound, fmt = concat!("Expected `", $Token, "`")))]
		#[parse(error(name = FollowsXid, fmt = "Token ends with `XID_CONTINUE` and follows `XID_START` or `_`"))]
		$(
			#[parse(skip_if_tag = $skip_if_tag)]
		)?
		#[parse(and_try_with = Self::check)]
		pub struct $TokenName(
			pub Whitespace,

			#[parse(try_update_with = Self::parse)]
			pub ParserStr,
		);

		impl $TokenName {
			fn parse(s: &mut &str) -> Result<(), <Self as Parse>::Error> {
				*s = s.strip_prefix($Token).ok_or(<Self as Parse>::Error::NotFound)?;
				Ok(())
			}

			fn check(&mut self, parser: &mut Parser) -> Result<(), <Self as Parse>::Error> {
				// Note: This checks prevents matching `match` on `matches`
				{
					let token = parser.str(self.1);
					let remaining = parser.remaining();

					if token.ends_with(unicode_ident::is_xid_continue) &&
						remaining.starts_with(|ch: char| unicode_ident::is_xid_continue(ch))
					{
						return Err(<Self as Parse>::Error::FollowsXid);
					}
				}

				$(
					// TODO: Different error message?
					if parser.strip_prefix($must_not_follow).is_some() {
						return Err(<Self as Parse>::Error::NotFound);
					}
				)*

				Ok(())
			}
		}
	)*
}

decl_tokens! {
	InnerLineDoc = "//!";
	OuterLineDoc = "///";
	InnerBlockDoc = "/*!";
	OuterBlockDoc = "/**";

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
	// TODO: This means we can't parse `let _:A<>=B`, despite it being
	//       accepted by the compiler.
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
	AndTy = '&';
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
