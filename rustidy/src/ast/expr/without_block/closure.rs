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
	async_: Option<token::Async>,
	move_:  Option<token::Move>,
	params: ClosureParams,
	ret:    Option<ClosureRet>,
	// TODO: If we parsed a return type, we should error
	//       if this isn't a block expression.
	expr:   Box<Expression>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ClosureParams {
	NoParams(token::OrOr),
	WithParams((token::Or, Option<ClosureParameters>, token::Or)),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureRet {
	arrow: token::RArrow,
	#[parse(fatal)]
	ty:    TypeNoBounds,
}

/// `ClosureParameters`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameters {
	params: PunctuatedTrailing<ClosureParameter, token::Comma>,
}

/// `ClosureParameter`
pub type ClosureParameter = WithOuterAttributes<ClosureParameterInner>;

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameterInner {
	pat: PatternNoTopAlt,
	ty:  Option<ClosureParameterInnerTy>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ClosureParameterInnerTy {
	colon: token::Colon,
	ty:    Type,
}
