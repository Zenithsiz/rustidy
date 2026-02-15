//! Block expression

// Imports
use {
	crate::{
		attr::BracedWithInnerAttributes,
		expr::ExpressionWithoutBlock,
		stmt::{ExpressionStatement, ExpressionStatementWithBlock, ExpressionStatementWithoutBlock, Statement},
		token,
	},
	rustidy_ast_util::NotFollows,
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::{Parse, ParseError, Parser, ParserError, ParserTag},
	rustidy_print::Print,
	rustidy_util::{Arena, ArenaData, ArenaIdx, Whitespace},
};

/// `BlockExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a block expression")]
#[parse(skip_if_tag = ParserTag::SkipBlockExpression)]
#[expect(clippy::use_self, reason = "`Parse` derive macro doesn't support `Self`")]
pub struct BlockExpression(pub ArenaIdx<BlockExpression>);

impl ArenaData for BlockExpression {
	type Data = BracedWithInnerAttributes<Statements>;

	const ARENA: &'static Arena<Self> = &TYPE_ARENA;
}

static TYPE_ARENA: Arena<BlockExpression> = Arena::new();

/// `Statements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Format, Print)]
pub struct Statements {
	#[format(args = rustidy_format::vec::args_prefix_ws(Whitespace::set_cur_indent))]
	pub stmts:         Vec<Statement>,
	#[format(prefix_ws(expr = Whitespace::set_cur_indent, if = !self.stmts.is_empty()))]
	pub trailing_expr: Option<ExpressionWithoutBlock>,
}

impl Parse for Statements {
	type Error = StatementsError;

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let mut stmts = vec![];
		let trailing_expr = loop {
			// Note: Blocks usually take priority over expressions here, as `{} * a`
			//       parses as an empty block, followed by the expression `*a`, but
			//       this is not the case for field/method access and the question mark
			//       operator.
			if let Ok((expr, ..)) = parser.try_parse::<(
				ExpressionStatementWithBlock,
				NotFollows<token::Dot>,
				NotFollows<token::Question>,
			)>()? {
				stmts.push(Statement::Expression(ExpressionStatement::WithBlock(expr)));
				continue;
			}

			match parser.peek::<(ExpressionWithoutBlock, Option<token::Semi>)>()? {
				Ok(((expr, semi), peek_expr_state)) => match semi {
					Some(semi) => {
						parser.set_peeked(peek_expr_state);
						stmts.push(Statement::Expression(ExpressionStatement::WithoutBlock(
							ExpressionStatementWithoutBlock { expr, semi },
						)));
					},
					None => match parser.with_tag(ParserTag::SkipExpressionWithoutBlock, Parser::peek::<Statement>)? {
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
				Err(_) =>
					match parser.with_tag(ParserTag::SkipExpressionWithoutBlock, Parser::try_parse::<Statement>)? {
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
	ExpressionStatementWithBlock(
		ParserError<(
			ExpressionStatementWithBlock,
			NotFollows<token::Dot>,
			NotFollows<token::Question>,
		)>,
	),

	#[parse_error(transparent)]
	ExpressionWithoutBlock(ParserError<(ExpressionWithoutBlock, Option<token::Semi>)>),

	#[parse_error(transparent)]
	Statement(ParserError<Statement>),
}
