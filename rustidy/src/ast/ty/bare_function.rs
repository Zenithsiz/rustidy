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
	pub for_lifetimes: Option<ForLifetimes>,
	pub qualifiers:    Option<FunctionTypeQualifiers>,
	pub fn_:           token::Fn,
	#[parse(fatal)]
	pub params:        Parenthesized<Option<FunctionParametersMaybeNamedVariadic>>,
	pub ret:           Option<BareFunctionReturnType>,
}

/// `FunctionTypeQualifiers`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionTypeQualifiers {
	pub unsafe_: Option<token::Unsafe>,
	pub extern_: Option<ExternAbi>,
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
	pub name: Option<MaybeNamedParamInnerName>,
	pub ty:   Box<Type>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedParamInnerName {
	pub inner: MaybeNamedParamInnerNameInner,
	pub colon: token::Colon,
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
	pub params:   Punctuated<MaybeNamedParam, token::Comma>,
	pub comma:    token::Comma,
	pub variadic: WithOuterAttributes<token::DotDotDot>,
}

/// `BareFunctionReturnType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BareFunctionReturnType {
	pub arrow: token::RArrow,
	pub ty:    Box<TypeNoBounds>,
}
