//! Match expression

// Imports
use {
	super::Conditions,
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			delimited::Braced,
			expr::{Expression, ExpressionInner},
			pat::Pattern,
			token,
			with_attrs::{WithInnerAttributes, WithOuterAttributes},
		},
		parser::{ParseError, Parser, ParserError},
	},
	core::ops::ControlFlow,
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
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct MatchArms {
	pub arms: Vec<MatchArmWithExpr>,
}

impl Parse for MatchArms {
	type Error = MatchArmsError;

	fn name() -> Option<impl fmt::Display> {
		None::<!>
	}

	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let mut arms = vec![];
		loop {
			let Ok(arm) = parser.try_parse::<MatchArm>()? else {
				break;
			};
			let arrow = parser.parse::<token::FatArrow>()?;
			let expr = parser.parse::<Expression>()?;

			let (trailing_comma, control_flow) = parser
				.arenas()
				.get::<Expression>()
				.with_value::<Result<_, Self::Error>>(expr.0, |expr| match expr {
					ExpressionInner::WithoutBlock(_) => match parser.try_parse::<token::Comma>()? {
						Ok(trailing_comma) => Ok((Some(trailing_comma), ControlFlow::Continue(()))),
						Err(_) => Ok((None, ControlFlow::Break(()))),
					},
					ExpressionInner::WithBlock(_) => {
						let trailing_comma = parser.try_parse::<token::Comma>()?.ok();
						Ok((trailing_comma, ControlFlow::Continue(())))
					},
				})?;

			arms.push(MatchArmWithExpr {
				arm,
				arrow,
				expr,
				trailing_comma,
			});

			if control_flow.is_break() {
				break;
			}
		}

		Ok(Self { arms })
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
pub struct MatchArmWithExpr {
	pub arm:            MatchArm,
	pub arrow:          token::FatArrow,
	pub expr:           Expression,
	pub trailing_comma: Option<token::Comma>,
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
