//! Array type

// Imports
use {
	super::{Type, TypeNoBounds},
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			delimited::Parenthesized,
			ident::Identifier,
			item::function::{ExternAbi, ForLifetimes},
			punct::{Punctuated, PunctuatedTrailing},
			token,
			with_attrs::WithOuterAttributes,
		},
	},
};

/// `BareFunctionType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BareFunctionType {
	for_lifetimes: Option<ForLifetimes>,
	qualifiers:    Option<FunctionTypeQualifiers>,
	fn_:           token::Fn,
	#[parse(fatal)]
	params:        Parenthesized<Option<FunctionParametersMaybeNamedVariadic>>,
	ret:           Option<BareFunctionReturnType>,
}

/// `FunctionTypeQualifiers`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionTypeQualifiers {
	unsafe_: Option<token::Unsafe>,
	extern_: Option<ExternAbi>,
}

/// `FunctionParametersMaybeNamedVariadic`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum FunctionParametersMaybeNamedVariadic {
	Variadic(MaybeNamedFunctionParametersVariadic),
	Normal(MaybeNamedFunctionParameters),
}

/// `MaybeNamedFunctionParameters`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedFunctionParameters(PunctuatedTrailing<MaybeNamedParam, token::Comma>);

/// `MaybeNamedParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedParam(pub WithOuterAttributes<MaybeNamedParamInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedParamInner {
	name: Option<MaybeNamedParamInnerName>,
	ty:   Box<Type>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedParamInnerName {
	inner: MaybeNamedParamInnerNameInner,
	colon: token::Colon,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MaybeNamedParamInnerNameInner {
	Ident(Identifier),
	Underscore(token::Underscore),
}

/// `MaybeNamedFunctionParametersVariadic`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedFunctionParametersVariadic {
	// TODO: `fn(...)` is accepted by the rust compiler, but
	//       the reference demands at least 1 argument, should
	//       we allow it?
	params:   Punctuated<MaybeNamedParam, token::Comma>,
	comma:    token::Comma,
	variadic: WithOuterAttributes<token::DotDotDot>,
}

/// `BareFunctionReturnType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BareFunctionReturnType {
	arrow: token::RArrow,
	ty:    Box<TypeNoBounds>,
}
