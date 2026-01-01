//! Fn statements

// Imports
use crate::{
	Format,
	ast::{
		delimited::{Delimited, Parenthesized},
		expr::{BlockExpression, LiteralExpression, StringLiteral},
		ident::Identifier,
		lifetime::Lifetime,
		pat::PatternNoTopAlt,
		punct::PunctuatedTrailing,
		token,
		ty::{Type, TypePath},
		with_attrs::WithOuterAttributes,
	},
	parser::Parse,
	print::Print,
};

/// `Function`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a function")]
pub struct Function {
	pub qualifiers: FunctionQualifiers,
	pub fn_:        token::Fn,
	#[parse(fatal)]
	pub ident:      Identifier,
	pub generics:   Option<GenericParams>,
	pub params:     Parenthesized<Option<FunctionParameters>>,
	pub ret:        Option<FunctionReturnType>,
	pub where_:     Option<WhereClause>,
	pub body:       FunctionBody,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum FunctionBody {
	#[format(indent)]
	Expr(BlockExpression),
	Semi(token::Semi),
}

/// `FunctionQualifiers`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "function qualifiers")]
pub struct FunctionQualifiers {
	pub const_:  Option<token::Const>,
	pub async_:  Option<token::Async>,
	pub safety:  Option<ItemSafety>,
	pub extern_: Option<ExternAbi>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternAbi {
	pub extern_: token::Extern,
	pub abi:     Option<Abi>,
}

/// `Abi`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum Abi {
	String(StringLiteral),
	RawString(!),
}

/// `ItemSafety`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ItemSafety {
	Safe(token::Safe),
	Unsafe(token::Unsafe),
}

/// `FunctionParameters`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "function parameters")]
pub enum FunctionParameters {
	Full(FunctionParametersFull),
	#[parse(peek = (SelfParam, Option::<token::Comma>, token::ParenClose))]
	OnlySelf(FunctionParametersOnlySelf),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersOnlySelf {
	pub self_:          SelfParam,
	pub trailing_comma: Option<token::Comma>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFull {
	pub self_: Option<FunctionParametersFullSelf>,
	pub rest:  PunctuatedTrailing<FunctionParam, token::Comma>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFullSelf {
	pub self_: SelfParam,
	pub comma: token::Comma,
}

/// `FunctionParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParam(pub WithOuterAttributes<FunctionParamInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum FunctionParamInner {
	Pattern(FunctionParamPattern),
	CVariadic(token::DotDotDot),
	Type(Type),
}

/// `FunctionParamPattern`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParamPattern {
	pub pat:   PatternNoTopAlt,
	pub colon: token::Colon,
	#[parse(fatal)]
	pub ty:    Type,
}

/// `SelfParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct SelfParam(pub WithOuterAttributes<ShorthandOrTypedSelf>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ShorthandOrTypedSelf {
	Typed(TypedSelf),
	Shorthand(ShorthandSelf),
}

/// `ShorthandSelf`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ShorthandSelf {
	pub ref_:  Option<ShorthandSelfRef>,
	pub mut_:  Option<token::Mut>,
	pub self_: token::SelfLower,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ShorthandSelfRef {
	pub ref_:     token::And,
	#[parse(fatal)]
	pub lifetime: Option<Lifetime>,
}

/// `TypedSelf`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypedSelf {
	pub mut_:  Option<token::Mut>,
	pub self_: token::SelfLower,
	pub colon: token::Colon,
	pub ty:    Type,
}

/// `FunctionReturnType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "function return type")]
pub struct FunctionReturnType {
	pub arrow: token::RArrow,
	#[parse(fatal)]
	pub ty:    Type,
}

/// `GenericParams`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "generic parameters")]
pub struct GenericParams(pub Delimited<Option<PunctuatedTrailing<GenericParam, token::Comma>>, token::Lt, token::Gt>);

/// `GenericParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericParam(pub WithOuterAttributes<GenericParamInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum GenericParamInner {
	Lifetime(LifetimeParam),
	Type(TypeParam),
	Const(ConstParam),
}

/// `LifetimeParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a lifetime parameter")]
pub struct LifetimeParam {
	pub lifetime: Lifetime,
	pub bounds:   Option<LifetimeParamBounds>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeParamBounds {
	pub colon:  token::Colon,
	pub bounds: Option<LifetimeBounds>,
}

/// `LifetimeBounds`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeBounds(PunctuatedTrailing<Lifetime, token::Plus>);

/// `TypeParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a type parameter")]
pub struct TypeParam {
	pub ident:  Identifier,
	pub bounds: Option<TypeParamColonBounds>,
	pub eq_ty:  Option<TypeParamEqType>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamColonBounds {
	pub colon:  token::Colon,
	pub bounds: Option<TypeParamBounds>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamEqType {
	pub eq: token::Eq,
	pub ty: Box<Type>,
}

/// `TypeParamBounds`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamBounds(pub PunctuatedTrailing<TypeParamBound, token::Plus>);

/// `TypeParamBound`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TypeParamBound {
	Lifetime(Lifetime),
	Trait(TraitBound),
	UseBound(!),
}

/// `TraitBound`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TraitBound {
	Parenthesized(Parenthesized<TraitBoundInner>),
	Normal(TraitBoundInner),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBoundInner {
	pub prefix: Option<TraitBoundInnerPrefix>,
	pub path:   TypePath,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TraitBoundInnerPrefix {
	Question(token::Question),
	ForLifetimes(Box<ForLifetimes>),
}

/// `WhereClause`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct WhereClause {
	pub where_: token::Where,
	// TODO: The reference says that this can't have a trailing comma,
	//       but the compiler accepts it, so we do to.
	pub items:  Option<PunctuatedTrailing<WhereClauseItem, token::Comma>>,
}

/// `WhereClauseItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum WhereClauseItem {
	Lifetime(LifetimeWhereClauseItem),
	Type(TypeBoundWhereClauseItem),
}

/// `LifetimeWhereClauseItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeWhereClauseItem {
	pub lifetime: Lifetime,
	pub colon:    token::Colon,
	#[parse(fatal)]
	pub bounds:   LifetimeBounds,
}

/// `TypeBoundWhereClauseItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeBoundWhereClauseItem {
	pub for_lifetimes: Option<ForLifetimes>,
	pub ty:            Type,
	pub colon:         token::Colon,
	#[parse(fatal)]
	pub bounds:        Option<TypeParamBounds>,
}

/// `ForLifetimes`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ForLifetimes {
	pub for_:   token::For,
	pub params: GenericParams,
}

/// `ConstParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParam {
	pub const_: token::Const,
	pub ident:  Identifier,
	pub colon:  token::Colon,
	pub ty:     Box<Type>,
	pub eq:     Option<ConstParamEq>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParamEq {
	eq:   token::Eq,
	rest: ConstParamEqRest,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ConstParamEqRest {
	Eq(Box<BlockExpression>),
	Ident(Identifier),
	Literal(ConstParamEqRestLiteral),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParamEqRestLiteral {
	neg:  Option<token::Minus>,
	expr: Box<LiteralExpression>,
}
