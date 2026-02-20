//! Patterns

// Modules
pub mod range;

// Exports
pub use self::range::RangePattern;

// Imports
use {
	super::{
		attr::WithOuterAttributes,
		expr::{
			MacroInvocation,
			PathExpression,
			without_block::{TupleIndex, path::PathInExpression},
		},
		token,
		util::{Braced, Bracketed, Parenthesized},
	},
	core::fmt::Debug,
	rustidy_ast_literal::{ByteLiteral, LiteralExpression},
	rustidy_ast_util::{
		AtLeast1,
		Identifier,
		Punctuated,
		PunctuatedTrailing,
		at_least,
		delimited,
		punct,
	},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::{Parse, ParsePeeked, ParserError},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Pattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a pattern")]
pub struct Pattern {
	pub top_alt: Option<token::Or>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.top_alt.is_some()))]
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::SINGLE))]
	pub inner:   Punctuated<PatternNoTopAlt, token::Or>,
}

/// `PatternNoTopAlt`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum PatternNoTopAlt {
	Range(RangePattern),
	WithoutRange(PatternWithoutRange),
}

/// `PatternWithoutRange`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct WildcardPattern(token::Underscore);

/// `RestPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RestPattern(token::DotDot);

/// `GroupedPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct GroupedPattern(#[format(args = delimited::fmt_remove())]
Parenthesized<Box<Pattern>>);

/// `SlicePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct SlicePattern(#[format(args = delimited::fmt_remove())]
Bracketed<Option<SlicePatternItems>>);

/// `SlicePatternItems`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct SlicePatternItems(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	PunctuatedTrailing<Box<Pattern>, token::Comma>,
);

/// `PathPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct PathPattern(PathExpression);

/// `ReferencePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ReferencePattern {
	pub ref_: ReferencePatternRef,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub mut_: Option<token::Mut>,
	#[format(prefix_ws = match self.mut_.is_some() {
		true => Whitespace::SINGLE,
		false => Whitespace::REMOVE,
	})]
	pub pat:  Box<PatternWithoutRange>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ReferencePatternRef {
	And(token::And),
	AndAnd(token::AndAnd),
}

/// `StructPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPattern {
	pub top:   PathInExpression,
	#[format(indent)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	// TODO: Use `delimited::fmt_single_or_indent_if_non_blank`
	#[format(args = delimited::fmt_indent_if_non_blank())]
	pub items: Braced<Option<StructPatternElements>>,
}

/// `StructPatternElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructPatternElements {
	Fields(StructPatternElementsFields),
	EtCetera(StructPatternEtCetera),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternElementsFields {
	pub fields:    StructPatternFields,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub et_cetera: Option<StructPatternElementsFieldsEtCetera>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternElementsFieldsEtCetera {
	pub comma:     token::Comma,
	#[format(prefix_ws = Whitespace::INDENT)]
	pub et_cetera: Option<StructPatternEtCetera>,
}

/// `StructPatternFields`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternFields(
	#[format(args = punct::fmt(Whitespace::INDENT, Whitespace::REMOVE))]
	Punctuated<StructPatternField, token::Comma>,
);

/// `StructPatternField`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternField(pub WithOuterAttributes<StructPatternFieldInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructPatternFieldInner {
	TuplePat(StructPatternFieldTuplePat),
	IdentPat(StructPatternFieldIdentPat),
	Ident(StructPatternFieldIdent),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternFieldTuplePat {
	pub idx: TupleIndex,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot: token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat: Box<Pattern>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternFieldIdentPat {
	pub ident: Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot:   token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat:   Box<Pattern>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternFieldIdent {
	pub ref_:  Option<token::Ref>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.ref_.is_some()))]
	pub mut_:  Option<token::Mut>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.ref_.is_some() || self.mut_.is_some()))]
	pub ident: Identifier,
}

/// `StructPatternEtCetera`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternEtCetera(token::DotDot);

/// `TupleStructPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleStructPattern {
	pub top:   PathInExpression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::fmt_remove())]
	pub items: Parenthesized<Option<TupleStructItems>>,
}

/// `TupleStructItems`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleStructItems(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub PunctuatedTrailing<Box<Pattern>, token::Comma>,
);

/// `TuplePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TuplePattern(
	#[format(args = delimited::fmt_remove())]
	Parenthesized<Option<TuplePatternItems>>,
);

/// `TuplePatternItems`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum TuplePatternItems {
	Pats(TupleItemsPats),
	Pat(TupleItemsPat),
	Rest(RestPattern),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleItemsPat {
	pub pat:   Box<Pattern>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub comma: token::Comma,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleItemsPats {
	pub first:          Box<Pattern>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = at_least::fmt_prefix_ws(Whitespace::REMOVE))]
	pub rest:           AtLeast1<TupleItemsPatsPat>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub trailing_comma: Option<token::Comma>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleItemsPatsPat {
	pub comma: token::Comma,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat:   Box<Pattern>,
}

/// `LiteralPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LiteralPattern {
	pub minus:   Option<token::Minus>,
	pub literal: LiteralExpression,
}

impl ParsePeeked<ByteLiteral> for LiteralPattern {
	fn parse_from_with_peeked(_parser: &mut rustidy_parse::Parser, parsed: ByteLiteral) -> Result<Self, Self::Error> {
		Ok(Self {
			minus: None,
			literal: LiteralExpression::Byte(parsed),
		})
	}
}

/// `IdentifierPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = Pat(ParserError::<PatternNoTopAlt>), transparent))]
pub struct IdentifierPattern {
	pub ref_:  Option<token::Ref>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.ref_.is_some()))]
	pub mut_:  Option<token::Mut>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.ref_.is_some() || self.mut_.is_some()))]
	pub ident: Identifier,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub rest:  Option<IdentifierPatternRest>,
}

impl ParsePeeked<(Option<token::Ref>, Option<token::Mut>, Identifier, token::At)> for IdentifierPattern {
	fn parse_from_with_peeked(parser: &mut rustidy_parse::Parser, (ref_, mut_, ident, at): (Option<token::Ref>, Option<token::Mut>, Identifier, token::At),) -> Result<Self, Self::Error> {
		let pat = parser
			.parse::<PatternNoTopAlt>()
			.map_err(Self::Error::Pat)?;
		Ok(Self {
			ref_,
			mut_,
			ident,
			rest: Some(IdentifierPatternRest {
				at,
				pat: Box::new(pat)
			}),
		})
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct IdentifierPatternRest {
	pub at:  token::At,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat: Box<PatternNoTopAlt>,
}
