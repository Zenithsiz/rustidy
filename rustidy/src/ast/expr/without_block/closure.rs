//! Closure

// Imports
use {
	super::ExpressionWithoutBlockInner,
	crate::{
		Format,
		Parse,
		ParseRecursive,
		Print,
		ast::{
			delimited::Delimited,
			expr::Expression,
			pat::PatternNoTopAlt,
			punct::PunctuatedTrailing,
			token,
			ty::{Type, TypeNoBounds},
			with_attrs::WithOuterAttributes,
		},
	},
};

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(ParseRecursive, Format, Print)]
#[parse_recursive(root = Expression)]
#[parse_recursive(into_root = ExpressionWithoutBlockInner)]
#[parse_recursive(kind = "right")]
pub struct ClosureExpression {
	pub async_: Option<token::Async>,
	pub move_:  Option<token::Move>,
	pub params: ClosureParams,
	pub ret:    Option<ClosureRet>,
	// TODO: If we parsed a return type, we should error
	//       if this isn't a block expression.
	pub expr:   Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ClosureParams {
	NoParams(token::OrOr),
	WithParams(Delimited<Option<ClosureParameters>, token::Or, token::Or>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureRet {
	pub arrow: token::RArrow,
	#[parse(fatal)]
	pub ty:    TypeNoBounds,
}

/// `ClosureParameters`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameters(pub PunctuatedTrailing<ClosureParameter, token::Comma>);

/// `ClosureParameter`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameter(pub WithOuterAttributes<ClosureParameterInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameterInner {
	pub pat: PatternNoTopAlt,
	pub ty:  Option<ClosureParameterInnerTy>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameterInnerTy {
	pub colon: token::Colon,
	pub ty:    Type,
}
