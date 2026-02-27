//! Block expression

// Imports
use {
	crate::{
		attr::{self, BracedWithInnerAttributes},
		expr::ExpressionWithoutBlock,
		stmt::{
			ExpressionStatement,
			ExpressionStatementWithBlock,
			ExpressionStatementWithoutBlock,
			Statement,
			StatementInner,
		},
		token,
	},
	rustidy_ast_util::{AtLeast1, NotFollows, at_least},
	rustidy_format::{Format, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::{Parse, ParseError, Parser, ParserError, ParserTag},
	rustidy_print::Print,
	rustidy_util::{ArenaIdx, Whitespace, decl_arena},
};

/// `BlockExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a block expression")]
#[parse(skip_if_tag = ParserTag::SkipBlockExpression)]
#[format(args(ty = "BlockExpressionFmt"))]
pub struct BlockExpression(
	#[format(args = {
		let max_len = match args.allow_singleline {
			true => match &self.0.0.value.inner {
				// If we have any non-expression statements, never
				// use a single line for it.
				Some(Statements::Full(_)) => 0,
				_ => 50,
			},
			false => 0,
		};
		attr::with::fmt_braced_single_or_indent(true, max_len)
	})]
	pub ArenaIdx<BracedWithInnerAttributes<Option<Statements>>>,
);

impl Format<WhitespaceConfig, ()> for BlockExpression {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		_args: ()
	) -> rustidy_format::FormatOutput {
		self.format(
			ctx,
			prefix_ws,
			BlockExpressionFmt { allow_singleline: true }
		)
	}
}

pub struct BlockExpressionFmt {
	pub allow_singleline: bool,
}

decl_arena! { BracedWithInnerAttributes<Option<Statements>> }

/// `Statements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Formattable, Format, Print)]
pub enum Statements {
	OnlyExpr(ExpressionWithoutBlock),
	Full(StatementsFull),
}

impl Parse for Statements {
	type Error = StatementsError;

	#[coverage(on)]
	fn parse_from(parser: &mut Parser) -> Result<Self, Self::Error> {
		let mut stmts = None::<AtLeast1<_>>;
		let mut push_stmt = |stmt| match &mut stmts {
			Some(stmts) => stmts.rest.push(stmt),
			None => stmts = Some(AtLeast1::single(stmt)),
		};

		let stmts = loop {
			// Note: Blocks usually take priority over expressions here, as `{} * a`
			//       parses as an empty block, followed by the expression `*a`, but
			//       this is not the case for field/method access and the question mark
			//       operator.
			if let Ok((expr, ..)) = parser
				.try_parse::<(ExpressionStatementWithBlock, NotFollows<token::Dot>, NotFollows<token::Question>,)>()? {
				let stmt = StatementInner::Expression(ExpressionStatement::WithBlock(expr));
				push_stmt(Statement(ArenaIdx::new(stmt)));
				continue;
			}

			match parser
				.peek::<(ExpressionWithoutBlock, Option<token::Semi>)>()? {
				Ok(((expr, semi), peek_expr_pos)) => match semi {
					Some(semi) => {
						parser.set_pos(peek_expr_pos);
						let stmt = StatementInner::Expression(
							ExpressionStatement::WithoutBlock(ExpressionStatementWithoutBlock { expr, semi })
						);
						push_stmt(Statement(ArenaIdx::new(stmt)));
					},
					None => match parser.with_tag(
						ParserTag::SkipExpressionWithoutBlock,
						Parser::peek::<Statement>
					)? {
						//       since braced statement macros don't need a semi-colon, while expression ones do.
						//       Since both have the same length, we prefer statements to expressions if they have
						//       the same length here.
						Ok((stmt, peek_stmt_pos)) if peek_stmt_pos >= peek_expr_pos => {
							parser.set_pos(peek_stmt_pos);
							push_stmt(stmt);
						},
						_ => {
							parser.set_pos(peek_expr_pos);
							break match stmts {
								Some(stmts) => Self::Full(
									StatementsFull { stmts, trailing_expr: Some(expr) }
								),
								None => Self::OnlyExpr(expr),
							};
						},
					},
				},
				Err(_) => match parser.with_tag(
					ParserTag::SkipExpressionWithoutBlock,
					Parser::try_parse::<Statement>
				)? {
					Ok(stmt) => push_stmt(stmt),
					Err(err) => match stmts {
						Some(stmts) => break Self::Full(StatementsFull { stmts, trailing_expr: None }),
						None => return Err(StatementsError::Statement(err)),
					},
				},
			}
		};

		Ok(stmts)
	}
}

#[derive(derive_more::Debug, derive_more::From, ParseError)]
pub enum StatementsError {
	#[parse_error(transparent)]
	ExpressionStatementWithBlock(
		ParserError<(ExpressionStatementWithBlock, NotFollows<token::Dot>, NotFollows<token::Question>,)>,
	),

	#[parse_error(transparent)]
	ExpressionWithoutBlock(ParserError<(ExpressionWithoutBlock, Option<token::Semi>)>),

	#[parse_error(transparent)]
	Statement(ParserError<Statement>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Formattable, Format, Print)]
pub struct StatementsFull {
	#[format(args = at_least::fmt_prefix_ws(Whitespace::INDENT))]
	pub stmts:         AtLeast1<Statement>,
	#[format(prefix_ws = Whitespace::INDENT)]
	pub trailing_expr: Option<ExpressionWithoutBlock>,
}
