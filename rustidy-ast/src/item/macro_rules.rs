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
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, ParserTag},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `MacroRulesDefinition`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRulesDefinition {
	pub macro_rules: token::MacroRules,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:         token::Not,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:       Identifier,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(indent)]
	pub def:         MacroRulesDef,
}

/// `MacroRulesDef`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroRulesDef {
	Parens(MacroRulesDefParens),
	Brackets(MacroRulesDefBrackets),
	Braces(MacroRulesDefBraces),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRulesDefParens {
	#[format(args = delimited::fmt_preserve())]
	pub rules: Parenthesized<MacroRules>,
	pub semi:  token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRulesDefBrackets {
	#[format(args = delimited::fmt_preserve())]
	pub rules: Bracketed<MacroRules>,
	pub semi:  token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRulesDefBraces {
	#[format(args = delimited::fmt_preserve())]
	pub rules: Braced<MacroRules>,
}

/// `MacroRules`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRules(
	#[format(args = punct::fmt(Whitespace::PRESERVE, Whitespace::PRESERVE))]
	PunctuatedTrailing<MacroRule, token::Semi>,
);

/// `MacroRule`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRule {
	pub matcher:     MacroMatcher,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub arrow:       token::FatArrow,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub transcriber: MacroTranscriber,
}

/// `MacroMatcher`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroMatcher {
	#[format(args = delimited::fmt_preserve())]
	Parens(Parenthesized<MacroMatcherMatches>),
	#[format(args = delimited::fmt_preserve())]
	Brackets(Bracketed<MacroMatcherMatches>),
	#[format(args = delimited::fmt_preserve())]
	Braces(Braced<MacroMatcherMatches>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroMatcherMatches(
	#[format(args = rustidy_format::vec::args_prefix_ws(Whitespace::PRESERVE))]
	Vec<MacroMatch>,
);

/// `MacroMatch`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroMatchDollarIdent {
	pub dollar: token::Dollar,
	#[format(prefix_ws = Whitespace::PRESERVE)]
	pub ident:  MacroMatchDollarIdentInner,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::PRESERVE)]
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::PRESERVE)]
	pub spec:   MacroFragSpec,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroMatchDollarIdentInner {
	Raw(RawIdentifier),
	#[parse(with_tag = ParserTag::SkipTokenCrate)]
	IdentOrKw(IdentifierOrKeyword),
	Underscore(token::Underscore),
}

/// `MacroFragSpec`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroMatchDollarRep {
	pub dollar:  token::Dollar,
	#[format(prefix_ws = Whitespace::PRESERVE)]
	#[format(args = delimited::fmt_preserve())]
	pub matches: Parenthesized<MacroMatchDollarRepMatches>,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::PRESERVE)]
	pub rep_sep: Option<MacroRepSep>,
	#[format(prefix_ws = Whitespace::PRESERVE)]
	pub rep_op:  MacroRepOp,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroMatchDollarRepMatches(
	#[format(args = at_least::fmt_prefix_ws(Whitespace::PRESERVE))]
	pub AtLeast1<Box<MacroMatch>>,
);

/// `MacroRepSep`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRepSep(#[parse(with_tag = ParserTag::SkipDelimiters)]
#[parse(with_tag = ParserTag::SkipTokenStar)]
#[parse(with_tag = ParserTag::SkipTokenPlus)]
#[parse(with_tag = ParserTag::SkipTokenQuestion)]
Token);

/// `MacroRepOp`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroRepOp {
	Star(token::Star),
	Plus(token::Plus),
	Question(token::Question),
}

/// `MacroTranscriber`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroTranscriber(DelimTokenTree);
