//! Match expression

// Imports
use {
	super::{Expression, ExpressionWithBlock},
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			attr::InnerAttrOrDocComment,
			expr::ExpressionWithoutBlock,
			pat::Pattern,
			token,
			with_attrs::WithOuterAttributes,
		},
	},
};

/// `MatchExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a match expression")]
pub struct MatchExpression {
	match_:    token::Match,
	#[parse(fatal)]
	scrutinee: Box<Scrutinee>,
	open:      token::BracesOpen,
	attrs:     Vec<InnerAttrOrDocComment>,
	arms:      Option<MatchArms>,
	close:     token::BracesClose,
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
	arms: Vec<MatchArmWithExprNonLast>,
	last: Option<MatchArmWithExpr<Expression, Option<token::Comma>>>,
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
	arm:            MatchArm,
	arrow:          token::FatArrow,
	expr:           Box<E>,
	trailing_comma: C,
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
	pat:   Pattern,
	guard: Option<MatchArmGuard>,
}

/// `MatchArmGuard`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct MatchArmGuard {
	if_:  token::If,
	expr: Box<Expression>,
}
