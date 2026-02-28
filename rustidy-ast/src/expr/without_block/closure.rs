//! Closure

// Imports
use {
	crate::{
		attr::{self, WithOuterAttributes},
		expr::{Expression, ExpressionInner},
		item::function::ForLifetimes,
		pat::PatternNoTopAlt,
		ty::{Type, TypeNoBounds},
	},
	super::ExpressionWithoutBlockInner,
	ast_literal::token,
	ast_util::{Delimited, PunctuatedTrailing, delimited, punct},
	format::{Format, Formattable, WhitespaceFormat},
	parse::{Parse, ParseRecursive},
	print::Print,
	util::Whitespace,
};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Formattable, Format, Print)]
#[parse_recursive(root = ExpressionInner)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "right")]
pub struct ClosureExpression {
	pub for_:   Option<ForLifetimes>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.for_.is_some()))]
	pub async_: Option<token::Async>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if_ = self.for_.is_some() || self.async_.is_some()))]
	pub move_:  Option<token::Move>,
	#[format(prefix_ws(
		expr = Whitespace::SINGLE,
		if_ = self.for_.is_some() || self.async_.is_some() || self.move_.is_some()
	))]
	pub params: ClosureParams,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ret:    Option<ClosureRet>,
	// TODO: If we parsed a return type, we should error
	//       if this isn't a block expression.
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr:   Expression,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum ClosureParams {
	NoParams(token::OrOr),
	#[format(args = delimited::FmtRemove)]
	WithParams(Delimited<Option<ClosureParameters>, token::Or, token::Or>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ClosureRet {
	pub arrow: token::RArrow,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    TypeNoBounds,
}

/// `ClosureParameters`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ClosureParameters(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))]
	pub PunctuatedTrailing<ClosureParameter, token::Comma>,
);

/// `ClosureParameter`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ClosureParameter(
	#[format(args = attr::with::fmt(Whitespace::SINGLE))]
	pub WithOuterAttributes<ClosureParameterInner>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ClosureParameterInner {
	pub pat: PatternNoTopAlt,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub ty:  Option<ClosureParameterInnerTy>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ClosureParameterInnerTy {
	pub colon: token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Type,
}
