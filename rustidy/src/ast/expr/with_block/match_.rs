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
		parser::{ParseError, Parser, ParserError},
	},
	std::fmt,
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
// TODO: Simplify this to just `Vec<MatchArmWithExpr<Expression, Option<token::Comma>>` and
//       be careful parsing and formatting it?
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct MatchArms {
	pub arms: Vec<MatchArmWithExprNonLast>,
	pub last: Option<MatchArmWithExpr<Box<Expression>, Option<token::Comma>>>,
}

impl Parse for MatchArms {
	type Error = MatchArmsError;

	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let mut arms = vec![];
		let mut last = loop {
			let Ok(arm) = parser.try_parse::<MatchArm>()? else {
				break None;
			};
			let arrow = parser.parse::<token::FatArrow>()?;
			let expr = parser.parse::<Expression>()?;
			match expr {
				Expression::WithoutBlock(expr) => match parser.try_parse::<token::Comma>()? {
					Ok(trailing_comma) => arms.push(MatchArmWithExprNonLast::WithoutBlock(MatchArmWithExpr {
						arm,
						arrow,
						expr,
						trailing_comma,
					})),
					Err(_) =>
						break Some(MatchArmWithExpr {
							arm,
							arrow,
							expr: Box::new(Expression::WithoutBlock(expr)),
							trailing_comma: None,
						}),
				},
				Expression::WithBlock(expr) => {
					let trailing_comma = parser.try_parse::<token::Comma>()?.ok();
					arms.push(MatchArmWithExprNonLast::WithBlock(MatchArmWithExpr {
						arm,
						arrow,
						expr,
						trailing_comma,
					}));
				},
			}
		};

		// Move the last arm to `last` if it's empty and it fits there.
		if last.is_none() &&
			let Some(arm) = arms.pop()
		{
			match arm {
				MatchArmWithExprNonLast::WithoutBlock(_) => arms.push(arm),
				MatchArmWithExprNonLast::WithBlock(arm) =>
					last = Some(MatchArmWithExpr {
						arm:            arm.arm,
						arrow:          arm.arrow,
						expr:           Box::new(Expression::WithBlock(arm.expr)),
						trailing_comma: arm.trailing_comma,
					}),
			}
		}

		Ok(Self { arms, last })
	}
}

#[derive(derive_more::Debug, derive_more::From, ParseError)]
pub enum MatchArmsError {
	#[parse_error(transparent)]
	MatchArm(ParserError<MatchArm>),

	#[parse_error(transparent)]
	#[parse_error(fatal)]
	FatArrow(ParserError<token::FatArrow>),

	#[parse_error(transparent)]
	#[parse_error(fatal)]
	Expression(ParserError<Expression>),

	#[parse_error(transparent)]
	#[parse_error(fatal)]
	Comma(ParserError<token::Comma>),
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
	pub expr:           E,
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
