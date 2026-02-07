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
	rustidy_ast_util::{AtLeast1, Identifier, Punctuated, PunctuatedTrailing, at_least, punct},
	rustidy_format::Format,
	rustidy_parse::{Parse, ParsePeeked, ParserError},
	rustidy_print::Print,
};

/// `Pattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a pattern")]
pub struct Pattern {
	pub top_alt: Option<token::Or>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.top_alt.is_some()))]
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
	#[format(before_with = Format::prefix_ws_remove)]
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
	#[format(before_with = Format::prefix_ws_set_single)]
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
	#[format(before_with = Format::prefix_ws_remove)]
	pub et_cetera: Option<StructPatternElementsFieldsEtCetera>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternElementsFieldsEtCetera {
	pub comma:     token::Comma,
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
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
	#[format(before_with = Format::prefix_ws_remove)]
	pub dot: token::Colon,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub pat: Box<Pattern>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFieldIdentPat {
	pub ident: Identifier,
	#[format(before_with = Format::prefix_ws_remove)]
	pub dot:   token::Colon,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub pat:   Box<Pattern>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructPatternFieldIdent {
	pub ref_:  Option<token::Ref>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.ref_.is_some()))]
	pub mut_:  Option<token::Mut>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.ref_.is_some() || self.mut_.is_some()))]
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
	#[format(before_with = Format::prefix_ws_remove)]
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
	#[format(before_with = Format::prefix_ws_remove)]
	pub comma: token::Comma,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleItemsPats {
	pub first:          Box<Pattern>,
	#[format(before_with = Format::prefix_ws_remove)]
	#[format(and_with = at_least::format(Format::prefix_ws_remove))]
	pub rest:           AtLeast1<TupleItemsPatsPat>,
	#[format(before_with = Format::prefix_ws_remove)]
	pub trailing_comma: Option<token::Comma>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleItemsPatsPat {
	pub comma: token::Comma,
	#[format(before_with = Format::prefix_ws_set_single)]
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

impl ParsePeeked<ByteLiteral> for LiteralPattern {
	fn parse_from_with_peeked(_parser: &mut rustidy_parse::Parser, parsed: ByteLiteral) -> Result<Self, Self::Error> {
		Ok(Self {
			minus:   None,
			literal: LiteralExpression::Byte(parsed),
		})
	}
}

/// `IdentifierPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(error(name = Pat(ParserError::<PatternNoTopAlt>), transparent))]
pub struct IdentifierPattern {
	pub ref_:  Option<token::Ref>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.ref_.is_some()))]
	pub mut_:  Option<token::Mut>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.ref_.is_some() || self.mut_.is_some()))]
	pub ident: Identifier,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub rest:  Option<IdentifierPatternRest>,
}

impl ParsePeeked<(Option<token::Ref>, Option<token::Mut>, Identifier, token::At)> for IdentifierPattern {
	fn parse_from_with_peeked(
		parser: &mut rustidy_parse::Parser,
		(ref_, mut_, ident, at): (Option<token::Ref>, Option<token::Mut>, Identifier, token::At),
	) -> Result<Self, Self::Error> {
		let pat = parser.parse::<PatternNoTopAlt>().map_err(Self::Error::Pat)?;
		Ok(Self {
			ref_,
			mut_,
			ident,
			rest: Some(IdentifierPatternRest { at, pat: Box::new(pat) }),
		})
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct IdentifierPatternRest {
	pub at:  token::At,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub pat: Box<PatternNoTopAlt>,
}
