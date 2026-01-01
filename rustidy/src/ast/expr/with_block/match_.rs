//! Match expression

// Imports
use {
	super::{Conditions, Expression, ExpressionWithBlock},
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			delimited::Braced,
			expr::ExpressionWithoutBlock,
			pat::Pattern,
			token,
			with_attrs::{WithInnerAttributes, WithOuterAttributes},
		},
	},
};

/// `MatchExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a match expression")]
pub struct MatchExpression {
	pub match_:    token::Match,
	#[parse(fatal)]
	pub scrutinee: Box<Scrutinee>,
	pub arms:      Braced<WithInnerAttributes<Option<MatchArms>>>,
}

/// `Scrutinee`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Scrutinee(#[parse(with_tag = "skip:StructExpression")] Expression);

/// `MatchArms`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MatchArms {
	pub arms: Vec<MatchArmWithExprNonLast>,
	pub last: Option<MatchArmWithExpr<Expression, Option<token::Comma>>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum MatchArmWithExprNonLast {
	WithoutBlock(MatchArmWithExpr<ExpressionWithoutBlock, token::Comma>),
	WithBlock(MatchArmWithExpr<ExpressionWithBlock, Option<token::Comma>>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MatchArmWithExpr<E, C> {
	pub arm:            MatchArm,
	pub arrow:          token::FatArrow,
	pub expr:           Box<E>,
	pub trailing_comma: C,
}

/// `MatchArm`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MatchArm(pub WithOuterAttributes<MatchArmInner>);

#[derive(PartialEq, Eq, Clone, derive_more::Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a match arm")]
pub struct MatchArmInner {
	pub pat:   Pattern,
	pub guard: Option<MatchArmGuard>,
}

/// `MatchArmGuard`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MatchArmGuard {
	pub if_:  token::If,
	// TODO: The reference says this is just an expression, but
	//       that means we don't parse `Some(...) if let ...`, so
	//       instead we allow any conditions.
	pub cond: Conditions,
}
