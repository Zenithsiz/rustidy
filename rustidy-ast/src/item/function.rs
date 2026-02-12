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
	rustidy_ast_util::{Delimited, Follows, Identifier, PunctuatedTrailing, punct},
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
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.qualifiers.has_any()))]
	pub fn_:        token::Fn,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub ident:      Identifier,
	#[format(prefix_ws = Whitespace::remove)]
	pub generics:   Option<GenericParams>,
	#[format(prefix_ws = Whitespace::remove)]
	#[format(and_with = Parenthesized::format_remove)]
	pub params:     Parenthesized<Option<FunctionParameters>>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ret:        Option<FunctionReturnType>,
	#[format(prefix_ws = Whitespace::set_cur_indent)]
	pub where_:     Option<WhereClause>,
	#[format(prefix_ws = match self.body.is_semi() {
		true => Whitespace::remove,
		false => match self.where_.is_some() {
			true => Whitespace::set_cur_indent,
			false => Whitespace::set_single,
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
	#[format(prefix_ws = Whitespace::set_single)]
	pub async_:  Option<token::Async>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub safety:  Option<ItemSafety>,
	#[format(prefix_ws = Whitespace::set_single)]
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
	#[format(prefix_ws = Whitespace::set_single)]
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
	#[format(prefix_ws = Whitespace::remove)]
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
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.self_.is_some()))]
	#[format(and_with = punct::format_trailing(Whitespace::set_single, Whitespace::remove))]
	pub rest:  PunctuatedTrailing<FunctionParam, token::Comma>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionParametersFullSelf {
	pub self_: SelfParam,
	#[format(prefix_ws = Whitespace::remove)]
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
	#[format(prefix_ws = Whitespace::remove)]
	pub colon: token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
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
		true => Whitespace::set_single,
		false => Whitespace::remove,
	})]
	pub mut_:  Option<token::Mut>,
	#[format(prefix_ws = match self.ref_.as_ref().is_some_and(|ref_| ref_.lifetime.is_some()) || self.mut_.is_some() {
		true => Whitespace::set_single,
		false => Whitespace::remove,
	})]
	pub self_: token::SelfLower,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ShorthandSelfRef {
	pub ref_:     token::And,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::remove)]
	pub lifetime: Option<Lifetime>,
}

/// `TypedSelf`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypedSelf {
	pub mut_:  Option<token::Mut>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.mut_.is_some()))]
	pub self_: token::SelfLower,
	#[format(prefix_ws = Whitespace::remove)]
	pub colon: token::Colon,
	#[format(prefix_ws = Whitespace::set_single)]
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
	#[format(prefix_ws = Whitespace::set_single)]
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
	#[format(and_with = punct::format_trailing(Whitespace::set_single, Whitespace::remove))]
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
	#[format(prefix_ws = Whitespace::remove)]
	pub bounds:   Option<LifetimeParamBounds>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeParamBounds {
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::set_single)]
	pub bounds: Option<LifetimeBounds>,
}

/// `LifetimeBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct LifetimeBounds(
	#[format(and_with = punct::format_trailing(Whitespace::set_single, Whitespace::set_single))]
	PunctuatedTrailing<Lifetime, token::Plus>,
);

/// `TypeParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a type parameter")]
pub struct TypeParam {
	pub ident:  Identifier,
	#[format(prefix_ws = Whitespace::remove)]
	pub bounds: Option<TypeParamColonBounds>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub eq_ty:  Option<TypeParamEqType>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamColonBounds {
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::set_single)]
	pub bounds: Option<TypeParamBounds>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamEqType {
	pub eq: token::Eq,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ty: Box<Type>,
}

/// `TypeParamBounds`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeParamBounds(
	#[format(and_with = punct::format_trailing(Whitespace::set_single, Whitespace::set_single))]
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
			Some(TraitBoundInnerPrefix::Question(_)) => self.path.format(ctx, &mut Whitespace::remove),
			Some(TraitBoundInnerPrefix::ForLifetimes(_)) => self.path.format(ctx, &mut Whitespace::set_single),
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
	#[format(prefix_ws = Whitespace::set_cur_indent)]
	#[format(indent)]
	#[format(and_with = rustidy_format::format_option_with(punct::format_trailing(Whitespace::set_cur_indent, Whitespace::remove)))]
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
	#[format(prefix_ws = Whitespace::remove)]
	pub colon:    token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub bounds:   LifetimeBounds,
}

/// `TypeBoundWhereClauseItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TypeBoundWhereClauseItem {
	pub for_lifetimes: Option<ForLifetimes>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.for_lifetimes.is_some()))]
	pub ty:            Type,
	#[format(prefix_ws = Whitespace::remove)]
	pub colon:         token::Colon,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub bounds:        Option<TypeParamBounds>,
}

/// `ForLifetimes`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ForLifetimes {
	pub for_:   token::For,
	#[format(prefix_ws = Whitespace::remove)]
	pub params: GenericParams,
}

/// `ConstParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParam {
	pub const_: token::Const,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ident:  Identifier,
	#[format(prefix_ws = Whitespace::remove)]
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::set_single)]
	pub ty:     Box<Type>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub eq:     Option<ConstParamEq>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ConstParamEq {
	eq:   token::Eq,
	#[format(prefix_ws = Whitespace::set_single)]
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
	#[format(prefix_ws = Whitespace::remove)]
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
