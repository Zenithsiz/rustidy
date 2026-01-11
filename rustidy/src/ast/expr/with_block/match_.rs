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
			expr::{Expression, ExpressionInner, ExpressionWithBlock, ExpressionWithoutBlock},
			pat::Pattern,
			token,
			with_attrs::{WithInnerAttributes, WithOuterAttributes},
		},
		parser::{FromRecursiveRoot, ParseError, Parser, ParserError},
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


			// Note: Because of `match () { () => {} () => {} }` and `match () { () => {}?, () => {} }`
			//       requiring look-ahead after the block expressions, we need to parse both with and
			//       without block.
			let with_block_res = parser.peek::<(ExpressionWithBlock, Option<token::Comma>)>()?;
			let without_block_res = parser.peek::<(ExpressionWithoutBlock, Option<token::Comma>)>()?;
			let (expr, trailing_comma, control_flow) = match (with_block_res, without_block_res) {
				// If both parse, we only accept with block if the without block had no comma.
				(
					Ok(((expr_with_block, with_block_trailing_comma), with_block_peek_state)),
					Ok(((expr_without_block, without_block_trailing_comma), without_block_peek_state)),
				) => match without_block_trailing_comma {
					Some(trailing_comma) => {
						parser.set_peeked(without_block_peek_state);

						let expr = Expression::from_recursive_root(ExpressionInner::from(expr_without_block), parser);
						(expr, Some(trailing_comma), ControlFlow::Continue(()))
					},
					None => {
						parser.set_peeked(with_block_peek_state);

						let expr = Expression::from_recursive_root(ExpressionInner::from(expr_with_block), parser);
						(expr, with_block_trailing_comma, ControlFlow::Continue(()))
					},
				},
				// If only one of them parses, we take it
				(Ok(((expr, trailing_comma), peek_state)), Err(_)) => {
					parser.set_peeked(peek_state);

					let expr = Expression::from_recursive_root(ExpressionInner::from(expr), parser);
					(expr, trailing_comma, ControlFlow::Continue(()))
				},
				(Err(_), Ok(((expr, trailing_comma), peek_state))) => {
					parser.set_peeked(peek_state);

					let expr = Expression::from_recursive_root(ExpressionInner::from(expr), parser);
					let control_flow = match trailing_comma.is_some() {
						true => ControlFlow::Continue(()),
						false => ControlFlow::Break(()),
					};

					(expr, trailing_comma, control_flow)
				},
				(Err(with_block), Err(without_block)) =>
					return Err(Self::Error::Expression {
						with_block,
						without_block,
					}),
			};

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
	ExpressionWithBlock(ParserError<(ExpressionWithBlock, Option<token::Comma>)>),

	#[parse_error(transparent)]
	#[parse_error(fatal)]
	ExpressionWithoutBlock(ParserError<(ExpressionWithoutBlock, Option<token::Comma>)>),

	#[parse_error(multiple)]
	#[parse_error(fatal)]
	Expression {
		with_block:    ParserError<(ExpressionWithBlock, Option<token::Comma>)>,
		without_block: ParserError<(ExpressionWithoutBlock, Option<token::Comma>)>,
	},

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
