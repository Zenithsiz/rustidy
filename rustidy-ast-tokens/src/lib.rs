//! Ast tokens

// Features
#![feature(
	never_type,
	coverage_attribute,
	yeet_expr,
	anonymous_lifetime_in_impl_trait,
	decl_macro,
	macro_metavar_expr
)]

// Imports
use {
	format::{Format, Formattable},
	parse::{Parse, ParserTag},
	print::Print,
	util::{AstStr, Whitespace},
};

pub macro decl_tokens(
	$ws:ident;
	$token:ident;
	$new:ident;

	$(
		$TokenName:ident = $Token:literal
		$( , ends_with_xid_continue $ends_with_xid_continue:tt )?
		$( , skip_if_tag $skip_if_tag:expr )?
		$( , must_not_follow $( $must_not_follow:literal ),* $(,)? )?
		;
	)*
) {
	$(
		#[derive(PartialEq, Eq, Clone, Debug)]
		#[derive(serde::Serialize, serde::Deserialize)]
		#[derive(Parse, Formattable, Format, Print)]
		#[parse(error(name = NotFound, fmt("Expected `{}`", $Token)))]
		$(
			#[parse(skip_if_tag = $skip_if_tag)]
		)?
		pub struct $TokenName {
			pub $ws: Whitespace,

			#[parse(try_update_with = Self::parse)]
			#[format(str)]
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

				// Note: This checks prevents matching `match` on `matches`
				$( ${ignore($ends_with_xid_continue)}
					if s.starts_with(|ch: char| unicode_ident::is_xid_continue(ch))
					{
						return Err(<Self as Parse>::Error::NotFound);
					}
				)?

				$(
					$(
						if s.starts_with($must_not_follow) {
							return Err(<Self as Parse>::Error::NotFound);
						}
					)*
				)?

				Ok(())
			}
		}

		#[expect(non_snake_case, reason = "This is a tuple constructor")]
		pub const fn $TokenName($ws: Whitespace, $token: AstStr) -> $TokenName {
			$TokenName {
				$ws, $token
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

	Super = "super", ends_with_xid_continue ();
	SelfLower = "self", ends_with_xid_continue ();
	SelfUpper = "Self", ends_with_xid_continue ();
	Crate = "crate", ends_with_xid_continue (), skip_if_tag ParserTag::SkipTokenCrate;
	DollarCrate = "$crate", ends_with_xid_continue ();

	As = "as", ends_with_xid_continue ();
	Async = "async", ends_with_xid_continue ();
	Attr = "attr", ends_with_xid_continue ();
	Auto = "auto", ends_with_xid_continue ();
	Await = "await", ends_with_xid_continue ();
	Break = "break", ends_with_xid_continue ();
	Const = "const", ends_with_xid_continue ();
	Continue = "continue", ends_with_xid_continue ();
	Derive = "derive", ends_with_xid_continue ();
	Do = "do", ends_with_xid_continue ();
	Dyn = "dyn", ends_with_xid_continue ();
	Else = "else", ends_with_xid_continue ();
	Enum = "enum", ends_with_xid_continue ();
	Extern = "extern", ends_with_xid_continue ();
	False = "false", ends_with_xid_continue ();
	Fn = "fn", ends_with_xid_continue ();
	For = "for", ends_with_xid_continue ();
	If = "if", ends_with_xid_continue ();
	Impl = "impl", ends_with_xid_continue ();
	In = "in", ends_with_xid_continue ();
	Let = "let", ends_with_xid_continue ();
	Loop = "loop", ends_with_xid_continue ();
	Macro = "macro", ends_with_xid_continue ();
	MacroRules = "macro_rules", ends_with_xid_continue ();
	Match = "match", ends_with_xid_continue ();
	Mod = "mod", ends_with_xid_continue ();
	Move = "move", ends_with_xid_continue ();
	Mut = "mut", ends_with_xid_continue ();
	Pub = "pub", ends_with_xid_continue ();
	Raw = "raw", ends_with_xid_continue ();
	Ref = "ref", ends_with_xid_continue ();
	Return = "return", ends_with_xid_continue ();
	Safe = "safe", ends_with_xid_continue ();
	Static = "static", ends_with_xid_continue ();
	Struct = "struct", ends_with_xid_continue ();
	Trait = "trait", ends_with_xid_continue ();
	True = "true", ends_with_xid_continue ();
	Try = "try", ends_with_xid_continue ();
	Type = "type", ends_with_xid_continue ();
	Union = "union", ends_with_xid_continue ();
	Unsafe = "unsafe", ends_with_xid_continue ();
	Use = "use", ends_with_xid_continue ();
	Where = "where", ends_with_xid_continue ();
	While = "while", ends_with_xid_continue ();
	Yeet = "yeet", ends_with_xid_continue ();

	// Macro frag spec
	Block = "block", ends_with_xid_continue ();
	Expr = "expr", ends_with_xid_continue ();
	Expr2021 = "expr_2021", ends_with_xid_continue ();
	Ident = "ident", ends_with_xid_continue ();
	Item = "item", ends_with_xid_continue ();
	Lifetime = "lifetime", ends_with_xid_continue ();
	Literal = "literal", ends_with_xid_continue ();
	Meta = "meta", ends_with_xid_continue ();
	Pat = "pat", ends_with_xid_continue ();
	PatParam = "pat_param", ends_with_xid_continue ();
	Path = "path", ends_with_xid_continue ();
	Stmt = "stmt", ends_with_xid_continue ();
	Tt = "tt", ends_with_xid_continue ();
	Ty = "ty", ends_with_xid_continue ();
	Vis = "vis", ends_with_xid_continue ();

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
