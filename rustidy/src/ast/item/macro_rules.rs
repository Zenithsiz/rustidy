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
		ident::{Identifier, IdentifierOrKeyword, ident_or_keyword::RawIdentifier},
		punct::PunctuatedTrailing,
		token::{self, Token},
	},
};

/// `MacroRulesDefinition`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefinition {
	pub macro_rules: token::MacroRules,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_remove)]
	pub not:         token::Not,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:       Identifier,
	#[format(and_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	pub def:         MacroRulesDef,
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
	pub rules: Parenthesized<MacroRules>,
	pub semi:  token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefBrackets {
	pub rules: Bracketed<MacroRules>,
	pub semi:  token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroRulesDefBraces {
	pub rules: Braced<MacroRules>,
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
	pub matcher:     MacroMatcher,
	#[parse(fatal)]
	pub arrow:       token::FatArrow,
	pub transcriber: MacroTranscriber,
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

	// Note: The reference says we shouldn't allow `$` here, but
	//       the compiler does, so we do as well, just with lower
	//       priority
	Dollar(token::Dollar),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MacroMatchDollarIdent {
	pub dollar: token::Dollar,
	pub ident:  MacroMatchDollarIdentInner,
	#[parse(fatal)]
	pub colon:  token::Colon,
	pub spec:   MacroFragSpec,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MacroMatchDollarIdentInner {
	Raw(RawIdentifier),
	#[parse(with_tag = "skip:`crate`")]
	IdentOrKw(IdentifierOrKeyword),
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
	pub dollar:  token::Dollar,
	pub matches: Parenthesized<AtLeast1<Box<MacroMatch>>>,
	#[parse(fatal)]
	pub rep_sep: Option<MacroRepSep>,
	pub rep_op:  MacroRepOp,
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
