//! Attributes

// Imports
use {
	super::{
		delimited::{Braced, Bracketed, Parenthesized},
		expr::Expression,
		line::RemainingLine,
		path::SimplePath,
		token,
		whitespace::Whitespace,
	},
	crate::{Format, parser::Parse, print::Print},
	core::fmt::Debug,
};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum InnerAttrOrDocComment {
	Attr(InnerAttribute),
	DocComment(InnerDocComment),
}

/// `InnerAttribute`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an inner attribute")]
pub struct InnerAttribute {
	pound: token::Pound,
	not:   token::Not,
	#[parse(fatal)]
	attr:  Bracketed<Attr>,
}

/// Inner Doc comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum InnerDocComment {
	InnerLine(InnerLineDoc),
	InnerBlock(!),
}

/// `INNER_LINE_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InnerLineDoc {
	#[format(whitespace)]
	whitespace: Whitespace,
	prefix:     token::raw::InnerLineDoc,
	comment:    RemainingLine,
}

/// Outer attribute or doc comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum OuterAttrOrDocComment {
	Attr(OuterAttribute),
	DocComment(OuterDocComment),
}

/// `OuterAttribute`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct OuterAttribute {
	pound: token::Pound,
	open:  Bracketed<Attr>,
}

/// Outer Doc comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum OuterDocComment {
	OuterLine(OuterLineDoc),
	OuterBlock(!),
}

/// `OUTER_LINE_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct OuterLineDoc {
	#[format(whitespace)]
	whitespace: Whitespace,
	prefix:     token::raw::OuterLineDoc,
	comment:    RemainingLine,
}

/// `Attr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Attr {
	// TODO: Unsafe attribute
	path:  SimplePath,
	input: Option<AttrInput>,
}

/// `AttrInput`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AttrInput {
	DelimTokenTree(DelimTokenTree),
	EqExpr((token::Eq, Expression)),
}

/// `DelimTokenTree`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum DelimTokenTree {
	Parens(Parenthesized<DelimTokenTreeInner>),
	Brackets(Bracketed<DelimTokenTreeInner>),
	Braces(Braced<DelimTokenTreeInner>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct DelimTokenTreeInner(#[parse(fatal)] Vec<TokenTree>);

/// `TokenTree`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TokenTree {
	Tree(DelimTokenTree),
	Tokens(Vec<TokenNonDelimited>),
}

/// `Token` except delimiters
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TokenNonDelimited(#[parse(with_tag = "skip:Delimiters")] token::Token);
