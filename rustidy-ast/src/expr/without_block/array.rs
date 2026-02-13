//! Array

// Imports
use {
	crate::{expr::Expression, token, util::Bracketed},
	rustidy_ast_util::punct::PunctuatedTrailing,
	rustidy_format::{Format, FormatFn, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ArrayExpression`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "an array expression")]
#[format(with = Self::format)]
pub struct ArrayExpression(Bracketed<Option<ArrayElements>>);

impl ArrayExpression {
	/// Formats all `values` within a single-line.
	fn format_single_line(
		values: &mut PunctuatedTrailing<Expression, token::Comma>,
		ctx: &mut rustidy_format::Context,
		prefix_ws: &mut impl FormatFn<Whitespace>,
	) {
		values.punctuated.first.format(ctx, prefix_ws);
		for (comma, value) in &mut values.punctuated.rest {
			comma.format(ctx, &mut Whitespace::remove);
			comma.format(ctx, &mut Whitespace::remove);

			value.format(ctx, &mut Whitespace::set_single);
			value.format(ctx, &mut Whitespace::set_single);
		}

		values.trailing.format(ctx, &mut Whitespace::remove);
		values.trailing.format(ctx, &mut Whitespace::remove);
	}

	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: &mut impl FormatFn<Whitespace>) {
		match &mut self.0.value {
			Some(ArrayElements::Punctuated(values)) => {
				Self::format_single_line(values, ctx, prefix_ws);

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

				match is_single_line {
					// If we fit, remove whitespace after the `[` and before the `]`.
					// Also remove the trailing comma.
					true => {
						values.punctuated.first.format(ctx, &mut Whitespace::remove);
						values.trailing = None;

						self.0.suffix.format(ctx, &mut Whitespace::remove);
						self.0.suffix.format(ctx, &mut Whitespace::remove);
					},

					false => {
						ctx.with_indent(|ctx| {
							// Format the first value in each column as indentation
							let mut values_iter = values.values_mut();
							loop {
								let mut row_values = (&mut values_iter).take(cols.unwrap_or(1));
								let Some(first) = row_values.next() else { break };
								first.format(ctx, &mut Whitespace::set_cur_indent);
								first.format(ctx, &mut Whitespace::set_cur_indent);

								for value in row_values {
									value.format(ctx, &mut Whitespace::set_single);
									value.format(ctx, &mut Whitespace::set_single);
								}
							}
							drop(values_iter);

							// Then add a trailing comma if we had none
							if values.trailing.is_none() {
								values.trailing = Some(token::Comma::new());
							}
						});

						// Finally, close the indentation on the `]`
						self.0.suffix.format(ctx, &mut Whitespace::set_cur_indent);
						self.0.suffix.format(ctx, &mut Whitespace::set_cur_indent);
					},
				}
			},

			Some(ArrayElements::Repeat(_)) | None => {
				self.0.format_remove(ctx);
				self.0.format(ctx, &mut Whitespace::remove);
			},
		}
	}
}

/// `ArrayElements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ArrayElements {
	Repeat(ArrayElementsRepeat),
	Punctuated(PunctuatedTrailing<Expression, token::Comma>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ArrayElementsRepeat {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi:  token::Semi,
	#[format(prefix_ws = Whitespace::set_single)]
	pub count: Expression,
}
