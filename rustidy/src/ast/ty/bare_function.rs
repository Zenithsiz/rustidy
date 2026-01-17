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
			punct::{self, Punctuated, PunctuatedTrailing},
			token,
			with_attrs::WithOuterAttributes,
		},
	},
};

/// `BareFunctionType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BareFunctionType {
	pub for_lifetimes: Option<ForLifetimes>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.for_lifetimes.is_some()))]
	pub qualifiers:    Option<FunctionTypeQualifiers>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.for_lifetimes.is_some() || self.qualifiers.is_some()))]
	pub fn_:           token::Fn,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = Parenthesized::format_remove)]
	pub params:        Parenthesized<Option<FunctionParametersMaybeNamedVariadic>>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ret:           Option<BareFunctionReturnType>,
}

/// `FunctionTypeQualifiers`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionTypeQualifiers {
	pub unsafe_: Option<token::Unsafe>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.unsafe_.is_some()))]
	pub extern_: Option<ExternAbi>,
}

/// `FunctionParametersMaybeNamedVariadic`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum FunctionParametersMaybeNamedVariadic {
	Variadic(MaybeNamedFunctionParametersVariadic),
	Normal(MaybeNamedFunctionParameters),
}

/// `MaybeNamedFunctionParameters`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedFunctionParameters(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	PunctuatedTrailing<MaybeNamedParam, token::Comma>,
);

/// `MaybeNamedParam`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedParam(pub WithOuterAttributes<MaybeNamedParamInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedParamInner {
	pub name: Option<MaybeNamedParamInnerName>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.name.is_some()))]
	pub ty:   Box<Type>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedParamInnerName {
	pub inner: MaybeNamedParamInnerNameInner,
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon: token::Colon,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MaybeNamedParamInnerNameInner {
	Ident(Identifier),
	Underscore(token::Underscore),
}

/// `MaybeNamedFunctionParametersVariadic`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedFunctionParametersVariadic {
	// TODO: `fn(...)` is accepted by the rust compiler, but
	//       the reference demands at least 1 argument, should
	//       we allow it?
	#[format(and_with = punct::format(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	pub params:   Punctuated<MaybeNamedParam, token::Comma>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub comma:    token::Comma,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub variadic: WithOuterAttributes<token::DotDotDot>,
}

/// `BareFunctionReturnType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BareFunctionReturnType {
	pub arrow: token::RArrow,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:    Box<TypeNoBounds>,
}
