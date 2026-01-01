//! Block expression

// Imports
use crate::{
	Format,
	ast::{
		delimited::Braced,
		expr::ExpressionWithoutBlock,
		stmt::{ExpressionStatement, ExpressionStatementWithoutBlock, Statement},
		token,
		with_attrs::WithInnerAttributes,
	},
	parser::{Parse, ParseError, Parser, ParserError},
	print::Print,
};

/// `BlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a block expression")]
#[parse(skip_if_tag = "skip:BlockExpression")]
pub struct BlockExpression(pub Braced<WithInnerAttributes<Statements>>);

/// `Statements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct Statements {
	pub stmts:         Vec<Statement>,
	pub trailing_expr: Option<ExpressionWithoutBlock>,
}

impl Parse for Statements {
	type Error = StatementsError;

	fn name() -> Option<impl std::fmt::Display> {
		None::<!>
	}

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let mut stmts = vec![];
		let trailing_expr = loop {
			match parser
				.try_parse::<ExpressionWithoutBlock>()
				.map_err(Self::Error::ExpressionWithoutBlock)?
			{
				Ok(expr) => match parser.try_parse::<token::Semi>().map_err(Self::Error::Semi)? {
					Ok(semi) => {
						stmts.push(Statement::Expression(ExpressionStatement::WithoutBlock(
							ExpressionStatementWithoutBlock { expr, semi },
						)));
					},
					Err(_) => break Some(expr),
				},
				Err(_) => match parser
					.with_tag("skip:ExpressionWithoutBlock", Parser::try_parse::<Statement>)
					.map_err(Self::Error::Statement)?
				{
					Ok(stmt) => stmts.push(stmt),
					Err(_) => break None,
				},
			}
		};

		Ok(Self { stmts, trailing_expr })
	}
}

#[derive(derive_more::Debug, ParseError)]
pub enum StatementsError {
	#[parse_error(transparent)]
	ExpressionWithoutBlock(ParserError<ExpressionWithoutBlock>),

	#[parse_error(transparent)]
	Semi(ParserError<token::Semi>),

	#[parse_error(transparent)]
	Statement(ParserError<Statement>),
}
