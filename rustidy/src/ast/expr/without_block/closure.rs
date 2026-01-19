//! Closure

// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::ast::{
		delimited::Delimited,
		expr::{Expression, ExpressionInner},
		pat::PatternNoTopAlt,
		punct::{self, PunctuatedTrailing},
		token,
		ty::{Type, TypeNoBounds},
		with_attrs::WithOuterAttributes,
	},
	rustidy_format::Format,
	rustidy_parse::{Parse, ParseRecursive},
	rustidy_print::Print,
};

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "right")]
pub struct ClosureExpression {
	pub async_: Option<token::Async>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.async_.is_some()))]
	pub move_:  Option<token::Move>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.async_.is_some() || self.move_.is_some()))]
	pub params: ClosureParams,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ret:    Option<ClosureRet>,
	// TODO: If we parsed a return type, we should error
	//       if this isn't a block expression.
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr:   Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ClosureParams {
	NoParams(token::OrOr),
	#[format(and_with = Delimited::format_remove)]
	WithParams(Delimited<Option<ClosureParameters>, token::Or, token::Or>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureRet {
	pub arrow: token::RArrow,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:    TypeNoBounds,
}

/// `ClosureParameters`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameters(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	pub  PunctuatedTrailing<ClosureParameter, token::Comma>,
);

/// `ClosureParameter`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameter(pub WithOuterAttributes<ClosureParameterInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameterInner {
	pub pat: PatternNoTopAlt,
	#[format(and_with = Format::prefix_ws_remove)]
	pub ty:  Option<ClosureParameterInnerTy>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameterInnerTy {
	pub colon: token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:    Type,
}
