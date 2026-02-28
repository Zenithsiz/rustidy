//! Patterns

// Modules
pub mod range;

// Exports
pub use self::range::RangePattern;

// Imports
use {
	crate::{attr, util::FmtSingleOrIndent},
	super::{
		attr::WithOuterAttributes,
		expr::{
			MacroInvocation,
			PathExpression,
			without_block::{TupleIndex, path::PathInExpression},
		},
		util::{Braced, Bracketed, Parenthesized},
	},
	ast_literal::{ByteLiteral, ByteStringLiteral, Identifier, LiteralExpression},
	ast_util::{AtLeast1, Punctuated, PunctuatedTrailing, at_least, delimited, punct},
	core::fmt::Debug,
	format::{Format, Formattable, WhitespaceFormat},
	parse::{Parse, ParsePeeked, ParserError},
	print::Print,
	util::Whitespace,
};

/// `Pattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a pattern")]
pub struct Pattern {
	pub top_alt: Option<ast_token::Or>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.top_alt.is_some()))]
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::SINGLE))]
	pub inner:   Punctuated<PatternNoTopAlt, ast_token::Or>,
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
	#[parse(peek = ByteStringLiteral)]
	Literal(LiteralPattern),
	// TODO: Parse this for single identifiers too?
	#[parse(peek = (Option::<ast_token::Ref>, Option::<ast_token::Mut>, Identifier, ast_token::At))]
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
pub struct WildcardPattern(ast_token::Underscore);

/// `RestPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct RestPattern(ast_token::DotDot);

/// `GroupedPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct GroupedPattern(#[format(args = delimited::FmtRemove)] Parenthesized<Box<Pattern>>);

/// `SlicePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct SlicePattern(#[format(args = delimited::FmtRemove)] Bracketed<Option<SlicePatternItems>>);

/// `SlicePatternItems`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct SlicePatternItems(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	PunctuatedTrailing<Box<Pattern>, ast_token::Comma>,
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
	pub mut_: Option<ast_token::Mut>,
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
	And(ast_token::And),
	AndAnd(ast_token::AndAnd),
}

/// `StructPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPattern {
	pub top:   PathInExpression,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(args = delimited::fmt_single_or_indent_if_non_blank(
		50,
		FmtSingleOrIndent::Single,
		FmtSingleOrIndent::Indent
	))]
	pub items: Braced<Option<StructPatternElements>>,
}

/// `StructPatternElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtSingleOrIndent"))]
pub enum StructPatternElements {
	#[format(args = args)]
	Fields(StructPatternElementsFields),
	EtCetera(StructPatternEtCetera),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtSingleOrIndent"))]
pub struct StructPatternElementsFields {
	#[format(args = args)]
	pub fields:    StructPatternFields,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = args)]
	pub et_cetera: Option<StructPatternElementsFieldsEtCetera>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtSingleOrIndent"))]
pub struct StructPatternElementsFieldsEtCetera {
	pub comma:     ast_token::Comma,
	#[format(prefix_ws = args.prefix_ws())]
	pub et_cetera: Option<StructPatternEtCetera>,
}

/// `StructPatternFields`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtSingleOrIndent"))]
pub struct StructPatternFields(
	#[format(args = punct::fmt(args.prefix_ws(), Whitespace::REMOVE))]
	Punctuated<StructPatternField, ast_token::Comma>,
);

/// `StructPatternField`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternField(
	#[format(args = attr::with::fmt(Whitespace::INDENT))]
	pub WithOuterAttributes<StructPatternFieldInner>,
);

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
	pub dot: ast_token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat: Box<Pattern>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternFieldIdentPat {
	pub ident: Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub dot:   ast_token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat:   Box<Pattern>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternFieldIdent {
	pub ref_:  Option<ast_token::Ref>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.ref_.is_some()))]
	pub mut_:  Option<ast_token::Mut>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.ref_.is_some() || self.mut_.is_some()))]
	pub ident: Identifier,
}

/// `StructPatternEtCetera`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructPatternEtCetera(ast_token::DotDot);

/// `TupleStructPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleStructPattern {
	pub top:   PathInExpression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtRemove)]
	pub items: Parenthesized<Option<TupleStructItems>>,
}

/// `TupleStructItems`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleStructItems(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub PunctuatedTrailing<Box<Pattern>, ast_token::Comma>,
);

/// `TuplePattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TuplePattern(
	#[format(args = delimited::FmtRemove)]
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
	pub comma: ast_token::Comma,
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
	pub trailing_comma: Option<ast_token::Comma>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleItemsPatsPat {
	pub comma: ast_token::Comma,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat:   Box<Pattern>,
}

/// `LiteralPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct LiteralPattern {
	pub minus:   Option<ast_token::Minus>,
	pub literal: LiteralExpression,
}

impl ParsePeeked<ByteLiteral> for LiteralPattern {
	fn parse_from_with_peeked(_parser: &mut parse::Parser, parsed: ByteLiteral) -> Result<Self, Self::Error> {
		Ok(Self {
			minus: None,
			literal: LiteralExpression::Byte(parsed),
		})
	}
}

impl ParsePeeked<ByteStringLiteral> for LiteralPattern {
	fn parse_from_with_peeked(_parser: &mut parse::Parser, parsed: ByteStringLiteral) -> Result<Self, Self::Error> {
		Ok(Self {
			minus: None,
			literal: LiteralExpression::ByteString(parsed),
		})
	}
}

/// `IdentifierPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(error(name = Pat(ParserError::<PatternNoTopAlt>), transparent))]
pub struct IdentifierPattern {
	pub ref_:  Option<ast_token::Ref>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.ref_.is_some()))]
	pub mut_:  Option<ast_token::Mut>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.ref_.is_some() || self.mut_.is_some()))]
	pub ident: Identifier,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub rest:  Option<IdentifierPatternRest>,
}

impl ParsePeeked<(Option<ast_token::Ref>, Option<ast_token::Mut>, Identifier, ast_token::At)> for IdentifierPattern {
	fn parse_from_with_peeked(
		parser: &mut parse::Parser,
		(ref_, mut_, ident, at): (Option<ast_token::Ref>, Option<ast_token::Mut>, Identifier, ast_token::At),
	) -> Result<Self, Self::Error> {
		let pat = parser
			.parse::<PatternNoTopAlt>()
			.map_err(Self::Error::Pat)?;
		Ok(Self {
			ref_,
			mut_,
			ident,
			rest: Some(IdentifierPatternRest { at, pat: Box::new(pat) }),
		})
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct IdentifierPatternRest {
	pub at:  ast_token::At,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub pat: Box<PatternNoTopAlt>,
}
