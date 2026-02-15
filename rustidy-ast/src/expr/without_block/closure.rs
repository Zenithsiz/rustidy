//! Closure

// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::{
		attr::WithOuterAttributes,
		expr::{Expression, ExpressionInner},
		pat::PatternNoTopAlt,
		token,
		ty::{Type, TypeNoBounds},
	},
	rustidy_ast_util::{Delimited, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{Parse, ParseRecursive},
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "right")]
pub struct ClosureExpression {
	pub async_: Option<token::Async>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.async_.is_some()))]
	pub move_:  Option<token::Move>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.async_.is_some() || self.move_.is_some()))]
	pub params: ClosureParams,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ret:    Option<ClosureRet>,
	// TODO: If we parsed a return type, we should error
	//       if this isn't a block expression.
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:   Expression,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ClosureParams {
	NoParams(token::OrOr),
	#[format(args = delimited::FmtArgs::remove((), (), ()))]
	WithParams(Delimited<Option<ClosureParameters>, token::Or, token::Or>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureRet {
	pub arrow: token::RArrow,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    TypeNoBounds,
}

/// `ClosureParameters`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameters(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
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
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub ty:  Option<ClosureParameterInnerTy>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameterInnerTy {
	pub colon: token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Type,
}
