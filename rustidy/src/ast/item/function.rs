//! Fn statements

// Imports
use {
	crate::ast::{
		expr::{BlockExpression, LiteralExpression, StringLiteral, without_block::literal::RawStringLiteral},
		ident::Identifier,
		lifetime::Lifetime,
		pat::PatternNoTopAlt,
		token,
		ty::{Type, TypePath},
		util::Parenthesized,
		with_attrs::{self, WithOuterAttributes},
	},
	rustidy_ast_util::{Delimited, PunctuatedTrailing, punct},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Function`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a function")]
pub struct Function {
	pub qualifiers: FunctionQualifiers,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.qualifiers.has_any()))]
	pub fn_:        token::Fn,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:      Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics:   Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = Parenthesized::format_remove)]
	pub params:     Parenthesized<Option<FunctionParameters>>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ret:        Option<FunctionReturnType>,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:     Option<WhereClause>,
	#[format(and_with = |body: &mut FunctionBody, ctx| match body.is_semi() {
		true => body.prefix_ws_remove(ctx),
		false => match self.where_.is_some() {
			true => body.prefix_ws_set_indent(ctx, 0, false),
			false => body.prefix_ws_set_single(ctx),
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub async_:  Option<token::Async>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub safety:  Option<ItemSafety>,
	#[format(and_with = Format::prefix_ws_set_single)]
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
	#[format(and_with = Format::prefix_ws_set_single)]
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
	#[parse(peek = (SelfParam, Option::<token::Comma>, token::ParenClose))]
	OnlySelf(FunctionParametersOnlySelf),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersOnlySelf {
	pub self_:          SelfParam,
	#[format(and_with = Format::prefix_ws_remove)]
	pub trailing_comma: Option<token::Comma>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFull {
	pub self_: Option<FunctionParametersFullSelf>,
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub rest:  PunctuatedTrailing<FunctionParam, token::Comma>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFullSelf {
	pub self_: SelfParam,
	#[format(and_with = Format::prefix_ws_remove)]
	pub comma: token::Comma,
}

/// `FunctionParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParam(
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_single))]
	pub  WithOuterAttributes<FunctionParamInner>,
);

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
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon: token::Colon,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
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
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = match self.ref_.as_ref().is_some_and(|ref_| ref_.lifetime.is_some()) {
		true => Format::prefix_ws_set_single,
		false => Format::prefix_ws_remove,
	})]
	pub mut_:  Option<token::Mut>,
	#[format(and_with = match self.ref_.as_ref().is_some_and(|ref_| ref_.lifetime.is_some()) || self.mut_.is_some() {
		true => Format::prefix_ws_set_single,
		false => Format::prefix_ws_remove,
	})]
	pub self_: token::SelfLower,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ShorthandSelfRef {
	pub ref_:     token::And,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_remove)]
	pub lifetime: Option<Lifetime>,
}

/// `TypedSelf`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypedSelf {
	pub mut_:  Option<token::Mut>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.mut_.is_some()))]
	pub self_: token::SelfLower,
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon: token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:    Type,
}

/// `GenericParams`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "generic parameters")]
pub struct GenericParams(
	#[format(and_with = Delimited::format_remove)] pub Delimited<Option<GenericParamsInner>, token::Lt, token::Gt>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "generic parameters")]
pub struct GenericParamsInner(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	pub  PunctuatedTrailing<GenericParam, token::Comma>,
);

/// `GenericParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct GenericParam(
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_single))]
	pub  WithOuterAttributes<GenericParamInner>,
);

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
	#[format(and_with = Format::prefix_ws_remove)]
	pub bounds:   Option<LifetimeParamBounds>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeParamBounds {
	pub colon:  token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bounds: Option<LifetimeBounds>,
}

/// `LifetimeBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeBounds(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_set_single))]
	PunctuatedTrailing<Lifetime, token::Plus>,
);

/// `TypeParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a type parameter")]
pub struct TypeParam {
	pub ident:  Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub bounds: Option<TypeParamColonBounds>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub eq_ty:  Option<TypeParamEqType>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamColonBounds {
	pub colon:  token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bounds: Option<TypeParamBounds>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamEqType {
	pub eq: token::Eq,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty: Box<Type>,
}

/// `TypeParamBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamBounds(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_set_single))]
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
	Parenthesized(Parenthesized<TraitBoundInner>),
	Normal(TraitBoundInner),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(and_with = Self::format_prefix)]
pub struct TraitBoundInner {
	pub prefix: Option<TraitBoundInnerPrefix>,
	pub path:   TypePath,
}

impl TraitBoundInner {
	fn format_prefix(&mut self, ctx: &mut rustidy_format::Context) {
		match self.prefix {
			Some(TraitBoundInnerPrefix::Question(_)) => self.path.prefix_ws_remove(ctx),
			Some(TraitBoundInnerPrefix::ForLifetimes(_)) => self.path.prefix_ws_set_single(ctx),
			None => (),
		}
	}
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
	#[format(indent)]
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	#[format(and_with = rustidy_format::format_option_with(punct::format_trailing(Format::prefix_ws_set_cur_indent, Format::prefix_ws_remove)))]
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
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon:    token::Colon,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bounds:   LifetimeBounds,
}

/// `TypeBoundWhereClauseItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeBoundWhereClauseItem {
	pub for_lifetimes: Option<ForLifetimes>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.for_lifetimes.is_some()))]
	pub ty:            Type,
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon:         token::Colon,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bounds:        Option<TypeParamBounds>,
}

/// `ForLifetimes`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ForLifetimes {
	pub for_:   token::For,
	#[format(and_with = Format::prefix_ws_remove)]
	pub params: GenericParams,
}

/// `ConstParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParam {
	pub const_: token::Const,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:  Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon:  token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:     Box<Type>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub eq:     Option<ConstParamEq>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParamEq {
	eq:   token::Eq,
	#[format(and_with = Format::prefix_ws_set_single)]
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
	#[format(and_with = Format::prefix_ws_remove)]
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
pub struct UseBoundGenericArgs(pub Delimited<UseBoundGenericArgsInner, token::Lt, token::Gt>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct UseBoundGenericArgsInner(pub PunctuatedTrailing<UseBoundGenericArg, token::Comma>);

/// `UseBoundGenericArg`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum UseBoundGenericArg {
	Lifetime(Lifetime),
	Identifier(Identifier),
	SelfUpper(token::SelfUpper),
}
