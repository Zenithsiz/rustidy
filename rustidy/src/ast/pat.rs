//! Patterns

// Modules
pub mod range;

// Exports
pub use self::range::RangePattern;

// Imports
use {
	super::{
		expr::{
			MacroInvocation,
			PathExpression,
			without_block::{TupleIndex, path::PathInExpression},
		},
		token,
		util::{Braced, Bracketed, Parenthesized},
		with_attrs::WithOuterAttributes,
	},
	core::fmt::Debug,
	rustidy_ast_literal::{ByteLiteral, LiteralExpression},
	rustidy_ast_util::{AtLeast1, Identifier, Punctuated, PunctuatedTrailing, at_least, punct},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Pattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a pattern")]
pub struct Pattern {
	pub top_alt: Option<token::Or>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.top_alt.is_some()))]
	#[format(and_with = punct::format(Format::prefix_ws_set_single, Format::prefix_ws_set_single))]
	pub inner:   Punctuated<PatternNoTopAlt, token::Or>,
}

/// `PatternNoTopAlt`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum PatternNoTopAlt {
	Range(RangePattern),
	WithoutRange(PatternWithoutRange),
}

/// `PatternWithoutRange`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum PatternWithoutRange {
	Struct(StructPattern),
	TupleStruct(TupleStructPattern),
	Path(PathPattern),

	#[parse(peek = ByteLiteral)]
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
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct WildcardPattern(token::Underscore);

/// `RestPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct RestPattern(token::DotDot);

/// `GroupedPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GroupedPattern(#[format(and_with = Parenthesized::format_remove)] Parenthesized<Box<Pattern>>);

/// `SlicePattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SlicePattern(#[format(and_with = Bracketed::format_remove)] Bracketed<Option<SlicePatternItems>>);

/// `SlicePatternItems`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SlicePatternItems(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	PunctuatedTrailing<Box<Pattern>, token::Comma>,
);

/// `PathPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct PathPattern(PathExpression);

/// `ReferencePattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ReferencePattern {
	pub ref_: ReferencePatternRef,
	#[format(and_with = Format::prefix_ws_remove)]
	pub mut_: Option<token::Mut>,
	#[format(and_with = match self.mut_.is_some() {
		true => Format::prefix_ws_set_single,
		false => Format::prefix_ws_remove,
	})]
	pub pat:  Box<PatternWithoutRange>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ReferencePatternRef {
	And(token::And),
	AndAnd(token::AndAnd),
}

/// `StructPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPattern {
	pub top:   PathInExpression,
	#[format(indent)]
	#[format(and_with = Format::prefix_ws_set_single)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub items: Braced<Option<StructPatternElements>>,
}

/// `StructPatternElements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructPatternElements {
	Fields(StructPatternElementsFields),
	EtCetera(StructPatternEtCetera),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternElementsFields {
	pub fields:    StructPatternFields,
	#[format(and_with = Format::prefix_ws_remove)]
	pub et_cetera: Option<StructPatternElementsFieldsEtCetera>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternElementsFieldsEtCetera {
	pub comma:     token::Comma,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub et_cetera: Option<StructPatternEtCetera>,
}

/// `StructPatternFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFields(
	#[format(and_with = punct::format(Format::prefix_ws_set_cur_indent, Format::prefix_ws_remove))]
	Punctuated<StructPatternField, token::Comma>,
);

/// `StructPatternField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternField(pub WithOuterAttributes<StructPatternFieldInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructPatternFieldInner {
	TuplePat(StructPatternFieldTuplePat),
	IdentPat(StructPatternFieldIdentPat),
	Ident(StructPatternFieldIdent),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFieldTuplePat {
	pub idx: TupleIndex,
	#[format(and_with = Format::prefix_ws_remove)]
	pub dot: token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub pat: Box<Pattern>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFieldIdentPat {
	pub ident: Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub dot:   token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub pat:   Box<Pattern>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFieldIdent {
	pub ref_:  Option<token::Ref>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.ref_.is_some()))]
	pub mut_:  Option<token::Mut>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.ref_.is_some() || self.mut_.is_some()))]
	pub ident: Identifier,
}

/// `StructPatternEtCetera`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternEtCetera(token::DotDot);

/// `TupleStructPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStructPattern {
	pub top:   PathInExpression,
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = Parenthesized::format_remove)]
	pub items: Parenthesized<Option<TupleStructItems>>,
}

/// `TupleStructItems`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStructItems(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	pub  PunctuatedTrailing<Box<Pattern>, token::Comma>,
);

/// `TuplePattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TuplePattern(#[format(and_with = Parenthesized::format_remove)] Parenthesized<Option<TuplePatternItems>>);

/// `TuplePatternItems`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TuplePatternItems {
	Pats(TupleItemsPats),
	Pat(TupleItemsPat),
	Rest(RestPattern),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleItemsPat {
	pub pat:   Box<Pattern>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub comma: token::Comma,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleItemsPats {
	pub first:          Box<Pattern>,
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = at_least::format(Format::prefix_ws_remove))]
	pub rest:           AtLeast1<TupleItemsPatsPat>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub trailing_comma: Option<token::Comma>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleItemsPatsPat {
	pub comma: token::Comma,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub pat:   Box<Pattern>,
}

/// `LiteralPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LiteralPattern {
	pub minus:   Option<token::Minus>,
	pub literal: LiteralExpression,
}

/// `IdentifierPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IdentifierPattern {
	pub ref_:  Option<token::Ref>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.ref_.is_some()))]
	pub mut_:  Option<token::Mut>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.ref_.is_some() || self.mut_.is_some()))]
	pub ident: Identifier,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub rest:  Option<IdentifierPatternRest>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IdentifierPatternRest {
	pub at:  token::At,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub pat: Box<PatternNoTopAlt>,
}
