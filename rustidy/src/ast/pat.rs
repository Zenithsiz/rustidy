//! Patterns

// Imports
use {
	super::{
		at_least::AtLeast1,
		delimited::{Braced, Bracketed, Parenthesized},
		expr::{
			LiteralExpression,
			MacroInvocation,
			PathExpression,
			without_block::{literal::TupleIndex, path::PathInExpression},
		},
		ident::Identifier,
		punct::{Punctuated, PunctuatedTrailing},
		token,
		with_attrs::WithOuterAttributes,
	},
	crate::{Format, parser::Parse, print::Print},
	core::fmt::Debug,
};

/// `Pattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a pattern")]
pub struct Pattern {
	top_alt: Option<token::Or>,
	inner:   Punctuated<PatternNoTopAlt, token::Or>,
}

/// `PatternNoTopAlt`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum PatternNoTopAlt {
	WithoutRange(PatternWithoutRange),
	Range(!),
}

/// `PatternWithoutRange`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum PatternWithoutRange {
	Struct(StructPattern),
	TupleStruct(TupleStructPattern),
	Path(PathPattern),

	Literal(LiteralPattern),
	// TODO: Parse this for single identifiers too?
	#[parse(peek = (Option::<token::Ref>, Option::<token::Mut>, Identifier, token::At))]
	Ident(IdentifierPattern),
	Wildcard(WildcardPattern),
	Rest(RestPattern),
	Reference(ReferencePattern),
	Tuple(TuplePattern),
	Grouped(GroupedPattern),
	Slice(SlicePattern),

	#[parse(peek = MacroInvocation)]
	Macro(MacroInvocation),
}

/// `WildcardPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct WildcardPattern(token::Underscore);

/// `RestPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RestPattern(token::DotDot);

/// `GroupedPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GroupedPattern(Parenthesized<Box<Pattern>>);

/// `SlicePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SlicePattern(Bracketed<Option<SlicePatternItems>>);

/// `SlicePatternItems`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SlicePatternItems(PunctuatedTrailing<Box<Pattern>, token::Comma>);

/// `PathPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathPattern(PathExpression);

/// `ReferencePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ReferencePattern {
	ref_: ReferencePatternRef,
	mut_: Option<token::Mut>,
	pat:  Box<PatternWithoutRange>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ReferencePatternRef {
	And(token::And),
	AndAnd(token::AndAnd),
}

/// `StructPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPattern {
	top:   PathInExpression,
	items: Braced<Option<StructPatternElements>>,
}

/// `StructPatternElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructPatternElements {
	Fields(StructPatternElementsFields),
	EtCetera(StructPatternEtCetera),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternElementsFields {
	pub fields:    StructPatternFields,
	pub et_cetera: Option<StructPatternElementsFieldsEtCetera>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternElementsFieldsEtCetera {
	pub comma:     token::Comma,
	pub et_cetera: Option<StructPatternEtCetera>,
}

/// `StructPatternFields`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFields(Punctuated<StructPatternField, token::Comma>);

/// `StructPatternField`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternField(pub WithOuterAttributes<StructPatternFieldInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructPatternFieldInner {
	TuplePat(StructPatternFieldTuplePat),
	IdentPat(StructPatternFieldIdentPat),
	Ident(StructPatternFieldIdent),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFieldTuplePat {
	idx: TupleIndex,
	dot: token::Colon,
	pat: Box<Pattern>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFieldIdentPat {
	ident: Identifier,
	dot:   token::Colon,
	pat:   Box<Pattern>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFieldIdent {
	ref_:  Option<token::Ref>,
	mut_:  Option<token::Mut>,
	ident: Identifier,
}

/// `StructPatternEtCetera`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternEtCetera(token::DotDot);

/// `TupleStructPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStructPattern {
	top:   PathInExpression,
	items: Parenthesized<Option<TupleStructItems>>,
}

/// `TupleStructItems`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStructItems {
	items: PunctuatedTrailing<Box<Pattern>, token::Comma>,
}

/// `TuplePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TuplePattern(Parenthesized<Option<TuplePatternItems>>);

/// `TuplePatternItems`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TuplePatternItems {
	Pats(TupleItemsPats),
	Pat(TupleItemsPat),
	Rest(RestPattern),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleItemsPat {
	pat:   Box<Pattern>,
	comma: token::Comma,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleItemsPats {
	first:          Box<Pattern>,
	rest:           AtLeast1<(token::Comma, Box<Pattern>)>,
	trailing_comma: Option<token::Comma>,
}

/// `LiteralPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LiteralPattern {
	minus:   Option<token::Minus>,
	literal: LiteralExpression,
}

/// `IdentifierPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IdentifierPattern {
	ref_:  Option<token::Ref>,
	mut_:  Option<token::Mut>,
	ident: Identifier,
	rest:  Option<IdentifierPatternRest>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IdentifierPatternRest {
	at:  token::At,
	pat: Box<PatternNoTopAlt>,
}
