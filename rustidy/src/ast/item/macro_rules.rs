//! Macro rules

// Imports
use crate::{
	Format,
	Parse,
	Print,
	ast::{
		at_least::AtLeast1,
		attr::DelimTokenTree,
		delimited::{Braced, Bracketed, Parenthesized},
		ident::{Identifier, IdentifierOrKeyword},
		punct::PunctuatedTrailing,
		token::{self, Token},
	},
};

/// `MacroRulesDefinition`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefinition {
	macro_rules: token::MacroRules,
	#[parse(fatal)]
	not:         token::Not,
	ident:       Identifier,
	def:         MacroRulesDef,
}

/// `MacroRulesDef`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroRulesDef {
	Parens(MacroRulesDefParens),
	Brackets(MacroRulesDefBrackets),
	Braces(MacroRulesDefBraces),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefParens {
	rules: Parenthesized<MacroRules>,
	semi:  token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefBrackets {
	rules: Bracketed<MacroRules>,
	semi:  token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefBraces {
	rules: Braced<MacroRules>,
}

/// `MacroRules`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRules(PunctuatedTrailing<MacroRule, token::Semi>);

/// `MacroRule`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRule {
	matcher:     MacroMatcher,
	#[parse(fatal)]
	arrow:       token::FatArrow,
	transcriber: MacroTranscriber,
}

/// `MacroMatcher`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroMatcher {
	Parens(Parenthesized<Vec<MacroMatch>>),
	Brackets(Bracketed<Vec<MacroMatch>>),
	Braces(Braced<Vec<MacroMatch>>),
}

/// `MacroMatch`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroMatch {
	#[parse(with_tag = "skip:`$`")]
	#[parse(with_tag = "skip:Delimiters")]
	Token(Token),

	Matcher(MacroMatcher),

	DollarIdent(MacroMatchDollarIdent),
	DollarRep(MacroMatchDollarRep),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroMatchDollarIdent {
	dollar: token::Dollar,
	ident:  MacroMatchDollarIdentInner,
	#[parse(fatal)]
	colon:  token::Colon,
	spec:   MacroFragSpec,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroMatchDollarIdentInner {
	#[parse(with_tag = "skip:`crate`")]
	IdentOrKw(IdentifierOrKeyword),
	Raw(!),
	Underscore(token::Underscore),
}

/// `MacroFragSpec`
#[derive(PartialEq, Eq, Clone, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroMatchDollarRep {
	dollar:  token::Dollar,
	matches: Parenthesized<AtLeast1<Box<MacroMatch>>>,
	#[parse(fatal)]
	rep_sep: Option<MacroRepSep>,
	rep_op:  MacroRepOp,
}

/// `MacroRepSep`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRepSep(
	#[parse(with_tag = "skip:Delimiters")]
	#[parse(with_tag = "skip:`*`")]
	#[parse(with_tag = "skip:`+`")]
	#[parse(with_tag = "skip:`?`")]
	Token,
);

/// `MacroRepOp`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroRepOp {
	Star(token::Star),
	Plus(token::Plus),
	Question(token::Question),
}

/// `MacroTranscriber`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroTranscriber(DelimTokenTree);
