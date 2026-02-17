//! Array

// Imports
use {
	crate::{expr::Expression, token, util::Bracketed},
	rustidy_ast_util::punct::{PunctuatedRest, PunctuatedTrailing},
	rustidy_format::{Format, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ArrayExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
#[parse(name = "an array expression")]
pub struct ArrayExpression(Bracketed<Option<ArrayElements>>);

impl ArrayExpression {
	/// Formats all `values` within a single-line.
	fn format_single_line(
		prefix: &mut token::BracketOpen,
		values: &mut PunctuatedTrailing<Expression, token::Comma>,
		suffix: &mut token::BracketClose,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
	) {
		prefix.format(ctx, prefix_ws, &mut ());
		values.punctuated.first.format(ctx, Whitespace::REMOVE, &mut ());
		for PunctuatedRest { punct: comma, value } in &mut values.punctuated.rest {
			comma.format(ctx, Whitespace::REMOVE, &mut ());
			value.format(ctx, Whitespace::SINGLE, &mut ());
		}

		// TODO: Should we preserve the original comma by returning it?
		values.trailing = None;
		suffix.format(ctx, Whitespace::REMOVE, &mut ());
	}
}

impl Format<()> for ArrayExpression {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: WhitespaceConfig, (): &mut ()) {
		match &mut self.0.value {
			Some(ArrayElements::Punctuated(values)) => {
				Self::format_single_line(&mut self.0.prefix, values, &mut self.0.suffix, ctx, prefix_ws);

				// Then check if we can fit into a single line.
				// Note: If the user specified a number of columns, only
				//       put everything into a single line if they fit.
				// TODO: Should we even special case that?
				let cols = ctx.config().array_expr_cols;
				let has_newlines = values.has_newlines(ctx, true);
				let is_single_line = !has_newlines &&
					match cols {
						Some(cols) => cols >= values.values_len(),
						None => values.len(ctx, true) <= ctx.config().max_array_expr_len,
					};

				// If we don't fit in a single line, format it multi-line
				match is_single_line {
					true => (),
					false => {
						ctx.with_indent(|ctx| {
							// Format the first value in each column as indentation
							let mut values_iter = values.values_mut();
							loop {
								let mut row_values = (&mut values_iter).take(cols.unwrap_or(1));
								let Some(first) = row_values.next() else { break };
								first.format(ctx, Whitespace::CUR_INDENT, &mut ());

								for value in row_values {
									value.format(ctx, Whitespace::SINGLE, &mut ());
								}
							}
							drop(values_iter);

							if values.trailing.is_none() {
								values.trailing = Some(token::Comma::new());
							}
							values.trailing.format(ctx, Whitespace::REMOVE, &mut ());
						});

						// Finally, close the indentation on the `]`
						self.0.suffix.format(ctx, Whitespace::CUR_INDENT, &mut ());
					},
				}
			},

			Some(ArrayElements::Repeat(repeat)) => {
				self.0.prefix.format(ctx, prefix_ws, &mut ());
				repeat.format(ctx, Whitespace::REMOVE, &mut ());
				self.0.suffix.format(ctx, Whitespace::REMOVE, &mut ());
			},

			None => (),
		}
	}
}

/// `ArrayElements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
pub enum ArrayElements {
	Repeat(ArrayElementsRepeat),
	Punctuated(PunctuatedTrailing<Expression, token::Comma>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ArrayElementsRepeat {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:  token::Semi,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub count: Expression,
}
