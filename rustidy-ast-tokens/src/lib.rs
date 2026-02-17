//! Ast tokens

// Features
#![feature(
	never_type,
	coverage_attribute,
	yeet_expr,
	anonymous_lifetime_in_impl_trait,
	decl_macro
)]

// Imports
use {
	rustidy_format::{Format, Formattable},
	rustidy_parse::{Parse, Parser, ParserTag},
	rustidy_print::Print,
	rustidy_util::{AstStr, Whitespace},
};

pub macro decl_tokens(
	$ws:ident;
	$token:ident;
	$new:ident;

	$(
		$TokenName:ident = $Token:literal
		$( , skip_if_tag $skip_if_tag:expr )?
		$( , must_not_follow $( $must_not_follow:literal ),* $(,)? )?
		;
	)*
) {
	$(
		#[derive(PartialEq, Eq, Debug)]
		#[derive(serde::Serialize, serde::Deserialize)]
		#[derive(Parse, Formattable, Format, Print)]
		#[parse(error(name = NotFound, fmt("Expected `{}`", $Token)))]
		#[parse(error(name = FollowsXid, fmt = "Token ends with `XID_CONTINUE` and follows `XID_START` or `_`"))]
		$(
			#[parse(skip_if_tag = $skip_if_tag)]
		)?
		#[parse(and_try_with = Self::check)]
		pub struct $TokenName {
			pub $ws: Whitespace,

			#[parse(try_update_with = Self::parse)]
			pub $token: AstStr,
		}

		impl $TokenName {
			/// Creates a new token, not associated to the input
			pub fn $new() -> Self {
				Self {
					$ws: Whitespace::empty(),
					$token: AstStr::new($Token)
				}
			}

			fn parse(s: &mut &str) -> Result<(), <Self as Parse>::Error> {
				*s = s.strip_prefix($Token).ok_or(<Self as Parse>::Error::NotFound)?;
				Ok(())
			}

			fn check(&mut self, parser: &mut Parser) -> Result<(), <Self as Parse>::Error> {
				// Note: This checks prevents matching `match` on `matches`
				{
					let token = parser.str(&self.$token);
					let remaining = parser.remaining();

					if token.ends_with(unicode_ident::is_xid_continue) &&
						remaining.starts_with(|ch: char| unicode_ident::is_xid_continue(ch))
					{
						return Err(<Self as Parse>::Error::FollowsXid);
					}
				}

				$(
					$(
						// TODO: Different error message?
						if parser.strip_prefix($must_not_follow).is_some() {
							return Err(<Self as Parse>::Error::NotFound);
						}
					)*
				)?

				Ok(())
			}
		}

		impl Default for $TokenName {
			fn default() -> Self {
				Self::$new()
			}
		}
	)*
}

decl_tokens! {
	ws;
	token;
	new;

	InnerLineDoc = "//!";
	OuterLineDoc = "///";
	InnerBlockDoc = "/*!";
	OuterBlockDoc = "/**";

	Super = "super";
	SelfLower = "self";
	SelfUpper = "Self";
	Crate = "crate", skip_if_tag ParserTag::SkipTokenCrate;
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
	Eq = '=', must_not_follow '=', '>';
	// TODO: This means we can't parse `let _:A<>=B`, despite it being
	//       accepted by the compiler.
	Lt = '<', must_not_follow '=';
	Le = "<=";
	EqEq = "==";
	Ne = "!=";
	Ge = ">=";
	Gt = '>', must_not_follow '=';
	AndAnd = "&&";
	OrOr = "||";
	Not = '!', must_not_follow '=';
	Tilde = '~';
	Plus = '+', skip_if_tag ParserTag::SkipTokenPlus, must_not_follow '=';
	Minus = '-', must_not_follow '=', '>';
	Star = '*', skip_if_tag ParserTag::SkipTokenStar, must_not_follow '=';
	Slash = '/', must_not_follow '=';
	Percent = '%', must_not_follow '=';
	Caret = '^', must_not_follow '=';
	And = '&', must_not_follow '&', '=';
	AndTy = '&';
	Or = '|', must_not_follow '|', '=';
	Shl = "<<", must_not_follow '=';
	Shr = ">>", must_not_follow '=';
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
	Dot = '.', must_not_follow '.';
	DotDot = "..", must_not_follow '.', '=';
	DotDotDot = "...";
	DotDotEq = "..=";
	Comma = ',';
	Semi = ';';
	Colon = ':', must_not_follow ':';
	PathSep = "::";
	RArrow = "->";
	LArrow = "<-";
	FatArrow = "=>";
	Pound = '#';
	Dollar = '$', skip_if_tag ParserTag::SkipTokenDollar;
	Question = '?', skip_if_tag ParserTag::SkipTokenQuestion;
	Underscore = '_';
	Quote = '\'';
	DoubleQuote = '"';

	ParenOpen = '(', skip_if_tag ParserTag::SkipDelimiters;
	ParenClose = ')', skip_if_tag ParserTag::SkipDelimiters;
	BracketOpen = '[', skip_if_tag ParserTag::SkipDelimiters;
	BracketClose = ']', skip_if_tag ParserTag::SkipDelimiters;
	BracesOpen = '{', skip_if_tag ParserTag::SkipDelimiters;
	BracesClose = '}', skip_if_tag ParserTag::SkipDelimiters;

}
