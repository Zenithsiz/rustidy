//! Fn statements

// Imports
use {
	crate::{
		attr::WithOuterAttributes,
		expr::BlockExpression,
		lifetime::Lifetime,
		pat::PatternNoTopAlt,
		token,
		ty::{Type, TypePath},
		util::Parenthesized,
	},
	rustidy_ast_literal::{LiteralExpression, RawStringLiteral, StringLiteral},
	rustidy_ast_util::{Delimited, Follows, Identifier, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{Parse, ParsePeeked},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Function`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a function")]
pub struct Function {
	pub qualifiers: FunctionQualifiers,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.qualifiers.has_any()))]
	pub fn_:        token::Fn,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:      Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics:   Option<GenericParams>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub params:     Parenthesized<Option<FunctionParameters>>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ret:        Option<FunctionReturnType>,
	#[format(prefix_ws = Whitespace::CUR_INDENT)]
	pub where_:     Option<WhereClause>,
	#[format(prefix_ws = match self.body.is_semi() {
		true => Whitespace::REMOVE,
		false => match self.where_.is_some() {
			true => Whitespace::CUR_INDENT,
			false => Whitespace::SINGLE,
		}
	})]
	pub body:       FunctionBody,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum FunctionBody {
	Expr(BlockExpression),
	Semi(token::Semi),
}

/// `FunctionQualifiers`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "function qualifiers")]
pub struct FunctionQualifiers {
	pub const_:  Option<token::Const>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub async_:  Option<token::Async>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub safety:  Option<ItemSafety>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub extern_: Option<ExternAbi>,
}

impl FunctionQualifiers {
	/// Returns if any qualifiers exist
	#[must_use]
	pub const fn has_any(&self) -> bool {
		self.const_.is_some() || self.async_.is_some() || self.safety.is_some() || self.extern_.is_some()
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternAbi {
	pub extern_: token::Extern,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub abi:     Option<Abi>,
}

/// `Abi`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum Abi {
	String(StringLiteral),
	RawString(RawStringLiteral),
}

/// `ItemSafety`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ItemSafety {
	Safe(token::Safe),
	Unsafe(token::Unsafe),
}

/// `FunctionParameters`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "function parameters")]
pub enum FunctionParameters {
	Full(FunctionParametersFull),
	#[parse(peek = (SelfParam, Option::<token::Comma>, Follows::<token::ParenClose>))]
	OnlySelf(FunctionParametersOnlySelf),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersOnlySelf {
	pub self_:          SelfParam,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub trailing_comma: Option<token::Comma>,
}

impl ParsePeeked<(SelfParam, Option<token::Comma>, Follows<token::ParenClose>)> for FunctionParametersOnlySelf {
	fn parse_from_with_peeked(
		_parser: &mut rustidy_parse::Parser,
		(self_, trailing_comma, _): (SelfParam, Option<token::Comma>, Follows<token::ParenClose>),
	) -> Result<Self, Self::Error> {
		Ok(Self { self_, trailing_comma })
	}
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFull {
	pub self_: Option<FunctionParametersFullSelf>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.self_.is_some()))]
	#[format(args = punct::args(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub rest:  PunctuatedTrailing<FunctionParam, token::Comma>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFullSelf {
	pub self_: SelfParam,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub comma: token::Comma,
}

/// `FunctionParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParam(pub WithOuterAttributes<FunctionParamInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum FunctionParamInner {
	Pattern(FunctionParamPattern),
	CVariadic(token::DotDotDot),
	Type(Type),
}

/// `FunctionParamPattern`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParamPattern {
	pub pat:   PatternNoTopAlt,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon: token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Type,
}

/// `SelfParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SelfParam(pub WithOuterAttributes<ShorthandOrTypedSelf>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ShorthandOrTypedSelf {
	Typed(TypedSelf),
	Shorthand(ShorthandSelf),
}

/// `ShorthandSelf`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ShorthandSelf {
	pub ref_:  Option<ShorthandSelfRef>,
	#[format(prefix_ws = match self.ref_.as_ref().is_some_and(|ref_| ref_.lifetime.is_some()) {
		true => Whitespace::SINGLE,
		false => Whitespace::REMOVE,
	})]
	pub mut_:  Option<token::Mut>,
	#[format(prefix_ws = match self.ref_.as_ref().is_some_and(|ref_| ref_.lifetime.is_some()) || self.mut_.is_some() {
		true => Whitespace::SINGLE,
		false => Whitespace::REMOVE,
	})]
	pub self_: token::SelfLower,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ShorthandSelfRef {
	pub ref_:     token::And,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub lifetime: Option<Lifetime>,
}

/// `TypedSelf`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypedSelf {
	pub mut_:  Option<token::Mut>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.mut_.is_some()))]
	pub self_: token::SelfLower,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon: token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Type,
}

/// `FunctionReturnType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "function return type")]
pub struct FunctionReturnType {
	pub arrow: token::RArrow,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Type,
}

/// `GenericParams`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "generic parameters")]
pub struct GenericParams(
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub  Delimited<Option<GenericParamsInner>, token::Lt, token::Gt>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "generic parameters")]
pub struct GenericParamsInner(
	#[format(args = punct::args(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub  PunctuatedTrailing<GenericParam, token::Comma>,
);

/// `GenericParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericParam(pub WithOuterAttributes<GenericParamInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum GenericParamInner {
	Lifetime(LifetimeParam),
	Type(TypeParam),
	Const(ConstParam),
}

/// `LifetimeParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a lifetime parameter")]
pub struct LifetimeParam {
	pub lifetime: Lifetime,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub bounds:   Option<LifetimeParamBounds>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeParamBounds {
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bounds: Option<LifetimeBounds>,
}

/// `LifetimeBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeBounds(
	#[format(args = punct::args(Whitespace::SINGLE, Whitespace::SINGLE))] PunctuatedTrailing<Lifetime, token::Plus>,
);

/// `TypeParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a type parameter")]
pub struct TypeParam {
	pub ident:  Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub bounds: Option<TypeParamColonBounds>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq_ty:  Option<TypeParamEqType>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamColonBounds {
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bounds: Option<TypeParamBounds>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamEqType {
	pub eq: token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty: Box<Type>,
}

/// `TypeParamBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamBounds(
	#[format(args = punct::args(Whitespace::SINGLE, Whitespace::SINGLE))]
	pub  PunctuatedTrailing<TypeParamBound, token::Plus>,
);

/// `TypeParamBound`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TypeParamBound {
	Lifetime(Lifetime),
	Trait(TraitBound),
	UseBound(UseBound),
}

/// `TraitBound`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TraitBound {
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	Parenthesized(Parenthesized<TraitBoundInner>),
	Normal(TraitBoundInner),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBoundInner {
	pub prefix: Option<TraitBoundInnerPrefix>,
	#[format(prefix_ws(if = let Some(prefix) = &self.prefix, expr = match prefix {
		TraitBoundInnerPrefix::Question(_) => Whitespace::REMOVE,
		TraitBoundInnerPrefix::ForLifetimes(_) => Whitespace::SINGLE,
	}))]
	pub path:   TypePath,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TraitBoundInnerPrefix {
	Question(token::Question),
	ForLifetimes(Box<ForLifetimes>),
}

/// `WhereClause`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct WhereClause {
	pub where_: token::Where,
	// TODO: The reference says that this can't have a trailing comma,
	//       but the compiler accepts it, so we do to.
	#[format(prefix_ws = Whitespace::CUR_INDENT)]
	#[format(indent)]
	#[format(args = punct::args(Whitespace::CUR_INDENT, Whitespace::REMOVE))]
	pub items:  Option<PunctuatedTrailing<WhereClauseItem, token::Comma>>,
}

/// `WhereClauseItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum WhereClauseItem {
	Lifetime(LifetimeWhereClauseItem),
	Type(TypeBoundWhereClauseItem),
}

/// `LifetimeWhereClauseItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeWhereClauseItem {
	pub lifetime: Lifetime,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:    token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bounds:   LifetimeBounds,
}

/// `TypeBoundWhereClauseItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeBoundWhereClauseItem {
	pub for_lifetimes: Option<ForLifetimes>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.for_lifetimes.is_some()))]
	pub ty:            Type,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:         token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bounds:        Option<TypeParamBounds>,
}

/// `ForLifetimes`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ForLifetimes {
	pub for_:   token::For,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub params: GenericParams,
}

/// `ConstParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParam {
	pub const_: token::Const,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:  Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:     Box<Type>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:     Option<ConstParamEq>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParamEq {
	eq:   token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	rest: ConstParamEqRest,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ConstParamEqRest {
	Eq(Box<BlockExpression>),
	Ident(Identifier),
	Literal(ConstParamEqRestLiteral),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParamEqRestLiteral {
	neg:  Option<token::Minus>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	expr: Box<LiteralExpression>,
}

/// `UseBound`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseBound {
	pub use_: token::Use,
	#[parse(fatal)]
	pub args: UseBoundGenericArgs,
}

/// `UseBoundGenericArgs`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseBoundGenericArgs(
	#[format(args = delimited::FmtArgs::preserve((), (), ()))]
	pub  Delimited<UseBoundGenericArgsInner, token::Lt, token::Gt>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseBoundGenericArgsInner(
	#[format(args = punct::args(Whitespace::PRESERVE, Whitespace::PRESERVE))]
	pub  PunctuatedTrailing<UseBoundGenericArg, token::Comma>,
);

/// `UseBoundGenericArg`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum UseBoundGenericArg {
	Lifetime(Lifetime),
	Identifier(Identifier),
	SelfUpper(token::SelfUpper),
}
