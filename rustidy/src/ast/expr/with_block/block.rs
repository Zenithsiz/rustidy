//! Block expression

// Imports
use {
	crate::ast::{
		delimited::Braced,
		expr::ExpressionWithoutBlock,
		stmt::{ExpressionStatement, ExpressionStatementWithoutBlock, Statement},
		token,
		with_attrs::WithInnerAttributes,
	},
	rustidy_format::Format,
	rustidy_parse::{Parse, ParseError, Parser, ParserError},
	rustidy_print::Print,
};

/// `BlockExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a block expression")]
#[parse(skip_if_tag = "skip:BlockExpression")]
pub struct BlockExpression(
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub Braced<WithInnerAttributes<Statements>>,
);

/// `Statements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct Statements {
	#[format(and_with = rustidy_format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))]
	pub stmts:         Vec<Statement>,
	#[format(and_with(expr = Format::prefix_ws_set_cur_indent, if = !self.stmts.is_empty()))]
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
			match parser.peek::<(ExpressionWithoutBlock, Option<token::Semi>)>()? {
				Ok(((expr, semi), peek_expr_state)) => match semi {
					Some(semi) => {
						parser.set_peeked(peek_expr_state);
						stmts.push(Statement::Expression(ExpressionStatement::WithoutBlock(
							ExpressionStatementWithoutBlock { expr, semi },
						)));
					},
					None => match parser.with_tag("skip:ExpressionWithoutBlock", Parser::peek::<Statement>)? {
						// Note: On macros, we want to ensure we parse a statement macro instead of expression macro,
						//       since braced statement macros don't need a semi-colon, while expression ones do.
						//       Since both have the same length, we prefer statements to expressions if they have
						//       the same length here.
						Ok((stmt, peek_stmt_state)) if peek_stmt_state.ahead_of_or_equal(&peek_expr_state) => {
							parser.set_peeked(peek_stmt_state);
							stmts.push(stmt);
						},
						_ => {
							parser.set_peeked(peek_expr_state);
							break Some(expr);
						},
					},
				},
				Err(_) => match parser.with_tag("skip:ExpressionWithoutBlock", Parser::try_parse::<Statement>)? {
					Ok(stmt) => stmts.push(stmt),
					Err(_) => break None,
				},
			}
		};

		Ok(Self { stmts, trailing_expr })
	}
}

#[derive(derive_more::Debug, derive_more::From, ParseError)]
pub enum StatementsError {
	#[parse_error(transparent)]
	ExpressionWithoutBlock(ParserError<(ExpressionWithoutBlock, Option<token::Semi>)>),

	#[parse_error(transparent)]
	Statement(ParserError<Statement>),
}
