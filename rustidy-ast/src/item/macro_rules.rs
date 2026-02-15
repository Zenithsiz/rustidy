//! Macro rules

// Imports
use {
	crate::{
		attr::DelimTokenTree,
		token::{self, Token},
		util::{Braced, Bracketed, Parenthesized},
	},
	rustidy_ast_util::{
		AtLeast1,
		Identifier,
		IdentifierOrKeyword,
		PunctuatedTrailing,
		RawIdentifier,
		at_least,
		delimited,
		punct,
	},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `MacroRulesDefinition`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefinition {
	pub macro_rules: token::MacroRules,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::remove)]
	pub not:         token::Not,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ident:       Identifier,
	#[format(prefix_ws = Whitespace::set_single)]
	#[format(indent)]
	pub def:         MacroRulesDef,
}

/// `MacroRulesDef`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroRulesDef {
	Parens(MacroRulesDefParens),
	Brackets(MacroRulesDefBrackets),
	Braces(MacroRulesDefBraces),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefParens {
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	pub rules: Parenthesized<MacroRules>,
	pub semi:  token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefBrackets {
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	pub rules: Bracketed<MacroRules>,
	pub semi:  token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefBraces {
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	pub rules: Braced<MacroRules>,
}

/// `MacroRules`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRules(
	#[format(args = punct::FmtArgs::new(Whitespace::preserve, Whitespace::preserve))]
	PunctuatedTrailing<MacroRule, token::Semi>,
);

/// `MacroRule`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRule {
	pub matcher:     MacroMatcher,
	#[parse(fatal)]
	pub arrow:       token::FatArrow,
	pub transcriber: MacroTranscriber,
}

/// `MacroMatcher`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroMatcher {
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	Parens(Parenthesized<MacroMatcherMatches>),
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	Brackets(Bracketed<MacroMatcherMatches>),
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	Braces(Braced<MacroMatcherMatches>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroMatcherMatches(
	#[format(args = rustidy_format::vec::Args::from_prefix_ws(Whitespace::preserve))] Vec<MacroMatch>,
);

/// `MacroMatch`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroMatch {
	#[parse(with_tag = ParserTag::SkipTokenDollar)]
	#[parse(with_tag = ParserTag::SkipDelimiters)]
	Token(Token),

	Matcher(MacroMatcher),

	DollarIdent(MacroMatchDollarIdent),
	DollarRep(MacroMatchDollarRep),

	// Note: The reference says we shouldn't allow `$` here, but
	//       the compiler does, so we do as well, just with lower
	//       priority
	Dollar(token::Dollar),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroMatchDollarIdent {
	pub dollar: token::Dollar,
	pub ident:  MacroMatchDollarIdentInner,
	#[parse(fatal)]
	pub colon:  token::Colon,
	pub spec:   MacroFragSpec,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroMatchDollarIdentInner {
	Raw(RawIdentifier),
	#[parse(with_tag = ParserTag::SkipTokenCrate)]
	IdentOrKw(IdentifierOrKeyword),
	Underscore(token::Underscore),
}

/// `MacroFragSpec`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroFragSpec {
	Block(token::Block),
	Expr(token::Expr),
	Expr2021(token::Expr2021),
	Ident(token::Ident),
	Item(token::Item),
	Lifetime(token::Lifetime),
	Literal(token::Literal),
	Meta(token::Meta),
	Pat(token::Pat),
	PatParam(token::PatParam),
	Path(token::Path),
	Stmt(token::Stmt),
	Tt(token::Tt),
	Ty(token::Ty),
	Vis(token::Vis),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroMatchDollarRep {
	pub dollar:  token::Dollar,
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	pub matches: Parenthesized<MacroMatchDollarRepMatches>,
	#[parse(fatal)]
	pub rep_sep: Option<MacroRepSep>,
	pub rep_op:  MacroRepOp,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroMatchDollarRepMatches(
	#[format(args = at_least::FmtArgs::from_prefix_ws(Whitespace::preserve))] pub AtLeast1<Box<MacroMatch>>,
);

/// `MacroRepSep`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRepSep(
	#[parse(with_tag = ParserTag::SkipDelimiters)]
	#[parse(with_tag = ParserTag::SkipTokenStar)]
	#[parse(with_tag = ParserTag::SkipTokenPlus)]
	#[parse(with_tag = ParserTag::SkipTokenQuestion)]
	Token,
);

/// `MacroRepOp`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroRepOp {
	Star(token::Star),
	Plus(token::Plus),
	Question(token::Question),
}

/// `MacroTranscriber`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroTranscriber(DelimTokenTree);
