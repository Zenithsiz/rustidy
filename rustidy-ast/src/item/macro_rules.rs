//! Macro rules

// Imports
use {
	crate::{attr::DelimTokenTree, util::{Braced, Bracketed, Parenthesized}},
	ast_literal::{Identifier, IdentifierOrKeyword, RawIdentifier, Token},
	ast_util::{AtLeast1, PunctuatedTrailing, at_least, delimited, punct},
	format::{Format, Formattable, WhitespaceFormat},
	parse::{Parse, ParserTag},
	print::Print,
	util::Whitespace,
};

/// `MacroRulesDefinition`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRulesDefinition {
	pub macro_rules: ast_token::MacroRules,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub not:         ast_token::Not,
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
	pub semi:  ast_token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRulesDefBrackets {
	#[format(args = delimited::fmt_preserve())]
	pub rules: Bracketed<MacroRules>,
	pub semi:  ast_token::Semi,
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
	PunctuatedTrailing<MacroRule, ast_token::Semi>,
);

/// `MacroRule`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroRule {
	pub matcher:     MacroMatcher,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub arrow:       ast_token::FatArrow,
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
	#[format(args = format::vec::args_prefix_ws(Whitespace::PRESERVE))]
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
	Dollar(ast_token::Dollar),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroMatchDollarIdent {
	pub dollar: ast_token::Dollar,
	#[format(prefix_ws = Whitespace::PRESERVE)]
	pub ident:  MacroMatchDollarIdentInner,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::PRESERVE)]
	pub colon:  ast_token::Colon,
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
	Underscore(ast_token::Underscore),
}

/// `MacroFragSpec`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroFragSpec {
	Block(ast_token::Block),
	Expr(ast_token::Expr),
	Expr2021(ast_token::Expr2021),
	Ident(ast_token::Ident),
	Item(ast_token::Item),
	Lifetime(ast_token::Lifetime),
	Literal(ast_token::Literal),
	Meta(ast_token::Meta),
	Pat(ast_token::Pat),
	PatParam(ast_token::PatParam),
	Path(ast_token::Path),
	Stmt(ast_token::Stmt),
	Tt(ast_token::Tt),
	Ty(ast_token::Ty),
	Vis(ast_token::Vis),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroMatchDollarRep {
	pub dollar:  ast_token::Dollar,
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
pub struct MacroRepSep(
	#[parse(with_tag = ParserTag::SkipDelimiters)]
	#[parse(with_tag = ParserTag::SkipTokenStar)]
	#[parse(with_tag = ParserTag::SkipTokenPlus)]
	#[parse(with_tag = ParserTag::SkipTokenQuestion)]
	Token,
);

/// `MacroRepOp`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MacroRepOp {
	Star(ast_token::Star),
	Plus(ast_token::Plus),
	Question(ast_token::Question),
}

/// `MacroTranscriber`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MacroTranscriber(DelimTokenTree);
