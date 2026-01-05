//! Attributes

// Imports
use {
	super::{
		delimited::{Braced, Bracketed, Parenthesized},
		expr::Expression,
		line::{RemainingBlockComment, RemainingLine},
		path::SimplePath,
		token,
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
	pub pound: token::Pound,
	pub not:   token::Not,
	#[parse(fatal)]
	pub attr:  Bracketed<Attr>,
}

/// Inner Doc comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum InnerDocComment {
	Line(InnerLineDoc),
	Block(InnerBlockDoc),
}

/// `INNER_LINE_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InnerLineDoc {
	pub prefix:  token::InnerLineDoc,
	pub comment: RemainingLine,
}

/// `INNER_BLOCK_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct InnerBlockDoc {
	pub prefix:  token::InnerBlockDoc,
	pub comment: RemainingBlockComment,
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
	pub pound: token::Pound,
	pub open:  Bracketed<Attr>,
}

/// Outer Doc comment
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum OuterDocComment {
	Line(OuterLineDoc),
	Block(OuterBlockDoc),
}

/// `OUTER_LINE_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct OuterLineDoc {
	pub prefix:  token::OuterLineDoc,
	pub comment: RemainingLine,
}

/// `OUTER_BLOCK_DOC`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct OuterBlockDoc {
	pub prefix:  token::OuterBlockDoc,
	// TODO: This should technically need whitespace before if we find `/**/**/*/`,
	//       but the reference doesn't seem to mention this, so we allow it.
	pub comment: RemainingBlockComment,
}

/// `Attr`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Attr {
	// TODO: Unsafe attribute
	pub path:  SimplePath,
	pub input: Option<AttrInput>,
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
