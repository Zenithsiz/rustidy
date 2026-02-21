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
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `BareFunctionType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct BareFunctionType {
	pub for_lifetimes: Option<ForLifetimes>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.for_lifetimes.is_some()))]
	pub qualifiers:    Option<FunctionTypeQualifiers>,
	#[format(
		prefix_ws(
			expr = Whitespace::SINGLE,
			if_ = self.for_lifetimes.is_some() || self
				.qualifiers
				.as_ref()
				.is_some_and(|qualifiers| qualifiers.unsafe_.is_some() || qualifiers.extern_.is_some())
		)
	)]
	pub fn_:           token::Fn,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::FmtRemove)]
	pub params:        Parenthesized<Option<FunctionParametersMaybeNamedVariadic>>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ret:           Option<BareFunctionReturnType>,
}

/// `FunctionTypeQualifiers`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct FunctionTypeQualifiers {
	pub unsafe_: Option<token::Unsafe>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.unsafe_.is_some()))]
	pub extern_: Option<ExternAbi>,
}

/// `FunctionParametersMaybeNamedVariadic`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum FunctionParametersMaybeNamedVariadic {
	Variadic(MaybeNamedFunctionParametersVariadic),
	Normal(MaybeNamedFunctionParameters),
}

/// `MaybeNamedFunctionParameters`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MaybeNamedFunctionParameters(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	PunctuatedTrailing<MaybeNamedParam, token::Comma>,
);

/// `MaybeNamedParam`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MaybeNamedParam(pub WithOuterAttributes<MaybeNamedParamInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MaybeNamedParamInner {
	pub name: Option<MaybeNamedParamInnerName>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.name.is_some()))]
	pub ty:   Box<Type>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MaybeNamedParamInnerName {
	pub inner: MaybeNamedParamInnerNameInner,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon: token::Colon,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum MaybeNamedParamInnerNameInner {
	Ident(Identifier),
	Underscore(token::Underscore),
}

/// `MaybeNamedFunctionParametersVariadic`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct MaybeNamedFunctionParametersVariadic {
	// TODO: `fn(...)` is accepted by the rust compiler, but
	//       the reference demands at least 1 argument, should
	//       we allow it?
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub params:   Punctuated<MaybeNamedParam, token::Comma>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub comma:    token::Comma,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub variadic: WithOuterAttributes<token::DotDotDot>,
}

/// `BareFunctionReturnType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct BareFunctionReturnType {
	pub arrow: token::RArrow,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Box<TypeNoBounds>,
}
