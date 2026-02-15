//! Array type

// Imports
use {
	super::{Type, TypeNoBounds},
	crate::{
		attr::WithOuterAttributes,
		item::function::{ExternAbi, ForLifetimes},
		token,
		util::Parenthesized,
	},
	rustidy_ast_util::{Identifier, Punctuated, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `BareFunctionType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BareFunctionType {
	pub for_lifetimes: Option<ForLifetimes>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.for_lifetimes.is_some()))]
	pub qualifiers:    Option<FunctionTypeQualifiers>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.for_lifetimes.is_some() || self.qualifiers.is_some()))]
	pub fn_:           token::Fn,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	pub params:        Parenthesized<Option<FunctionParametersMaybeNamedVariadic>>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ret:           Option<BareFunctionReturnType>,
}

/// `FunctionTypeQualifiers`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct FunctionTypeQualifiers {
	pub unsafe_: Option<token::Unsafe>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.unsafe_.is_some()))]
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
	#[format(args = punct::args(Whitespace::SINGLE, Whitespace::REMOVE))]
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
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.name.is_some()))]
	pub ty:   Box<Type>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MaybeNamedParamInnerName {
	pub inner: MaybeNamedParamInnerNameInner,
	#[format(prefix_ws = Whitespace::REMOVE)]
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
	#[format(args = punct::args(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub params:   Punctuated<MaybeNamedParam, token::Comma>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub comma:    token::Comma,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub variadic: WithOuterAttributes<token::DotDotDot>,
}

/// `BareFunctionReturnType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct BareFunctionReturnType {
	pub arrow: token::RArrow,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Box<TypeNoBounds>,
}
