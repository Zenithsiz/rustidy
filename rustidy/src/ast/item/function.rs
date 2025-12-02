//! Fn statements

// Imports
use crate::{
	Format,
	ast::{
		attr::OuterAttrOrDocComment,
		delimited::Parenthesized,
		expr::{BlockExpression, LiteralExpression, StringLiteral},
		ident::Ident,
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
	qualifiers: FunctionQualifiers,
	fn_:        token::Fn,
	#[parse(fatal)]
	ident:      Ident,
	generics:   Option<GenericParams>,
	params:     Parenthesized<Option<FunctionParameters>>,
	ret:        Option<FunctionReturnType>,
	where_:     Option<WhereClause>,
	body:       FunctionBody,
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
	const_:  Option<token::Const>,
	async_:  Option<token::Async>,
	safety:  Option<ItemSafety>,
	extern_: Option<ExternAbi>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternAbi {
	extern_: token::Extern,
	abi:     Option<Abi>,
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
	OnlySelf(FunctionParametersOnlySelf),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersOnlySelf {
	self_:          SelfParam,
	trailing_comma: Option<token::Comma>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFull {
	self_: Option<FunctionParametersFullSelf>,
	rest:  PunctuatedTrailing<FunctionParam, token::Comma>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFullSelf {
	self_: SelfParam,
	comma: token::Comma,
}

/// `FunctionParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "&self parameter")]
pub struct FunctionParam {
	attrs: Vec<OuterAttrOrDocComment>,
	inner: FunctionParamInner,
}

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
	pat:   PatternNoTopAlt,
	colon: token::Colon,
	#[parse(fatal)]
	ty:    Type,
}

/// `SelfParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "&self parameter")]
pub struct SelfParam {
	attrs: Vec<OuterAttrOrDocComment>,
	self_: ShorthandOrTypedSelf,
}

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
	ref_:  Option<ShorthandSelfRef>,
	mut_:  Option<token::Mut>,
	self_: token::SelfLower,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ShorthandSelfRef {
	ref_:     token::And,
	#[parse(fatal)]
	lifetime: Option<Lifetime>,
}

/// `TypedSelf`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypedSelf {
	mut_:  Option<token::Mut>,
	self_: token::SelfLower,
	colon: token::Colon,
	ty:    Type,
}

/// `FunctionReturnType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "function return type")]
pub struct FunctionReturnType {
	arrow: token::RArrow,
	#[parse(fatal)]
	ty:    Type,
}

/// `GenericParams`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "generic parameters")]
pub struct GenericParams {
	open:   token::Lt,
	#[parse(fatal)]
	params: Option<PunctuatedTrailing<GenericParam, token::Comma>>,
	close:  token::Gt,
}

/// `GenericParam`
pub type GenericParam = WithOuterAttributes<GenericParamInner>;

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
	lifetime: Lifetime,
	bounds:   Option<LifetimeParamBounds>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeParamBounds {
	colon:  token::Colon,
	bounds: Option<LifetimeBounds>,
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
	ident:  Ident,
	bounds: Option<TypeParamColonBounds>,
	eq_ty:  Option<TypeParamEqType>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamColonBounds {
	colon:  token::Colon,
	bounds: Option<TypeParamBounds>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamEqType {
	eq: token::Eq,
	ty: Box<Type>,
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
	prefix: Option<TraitBoundInnerPrefix>,
	path:   TypePath,
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
	where_: token::Where,
	// TODO: The reference says that this can't have a trailing comma,
	//       but the compiler accepts it, so we do to.
	items:  Option<PunctuatedTrailing<WhereClauseItem, token::Comma>>,
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
	lifetime: Lifetime,
	colon:    token::Colon,
	#[parse(fatal)]
	bounds:   LifetimeBounds,
}

/// `TypeBoundWhereClauseItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeBoundWhereClauseItem {
	for_lifetimes: Option<ForLifetimes>,
	ty:            Type,
	colon:         token::Colon,
	#[parse(fatal)]
	bounds:        Option<TypeParamBounds>,
}

/// `ForLifetimes`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ForLifetimes {
	for_:   token::For,
	params: GenericParams,
}

/// `ConstParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParam {
	const_: token::Const,
	ident:  Ident,
	colon:  token::Colon,
	ty:     Box<Type>,
	rest:   Option<ConstParamRest>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ConstParamRest {
	Eq((token::Eq, Box<BlockExpression>)),
	Ident(Ident),
	Literal((Option<token::Minus>, Box<LiteralExpression>)),
}
