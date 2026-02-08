//! Array

// Imports
use {
	crate::{expr::Expression, token, util::Bracketed},
	rustidy_ast_util::punct::PunctuatedTrailing,
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
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
	) {
		values.punctuated.first.format(ctx);
		for (comma, value) in &mut values.punctuated.rest {
			comma.prefix_ws_remove(ctx);
			comma.format(ctx);

			value.prefix_ws_set_single(ctx);
			value.format(ctx);
		}

		values.trailing.prefix_ws_remove(ctx);
		values.trailing.format(ctx);
	}

	fn format(&mut self, ctx: &mut rustidy_format::Context) {
		match &mut self.0.value {
			Some(ArrayElements::Punctuated(values)) => {
				Self::format_single_line(values, ctx);

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
						values.punctuated.first.prefix_ws_remove(ctx);
						values.trailing = None;

						self.0.suffix.prefix_ws_remove(ctx);
						self.0.suffix.format(ctx);
					},

					false => {
						ctx.with_indent(|ctx| {
							// Format the first value in each column as indentation
							let mut values_iter = values.values_mut();
							loop {
								let mut row_values = (&mut values_iter).take(cols.unwrap_or(1));
								let Some(first) = row_values.next() else { break };
								first.prefix_ws_set_cur_indent(ctx);
								first.format(ctx);

								for value in row_values {
									value.prefix_ws_set_single(ctx);
									value.format(ctx);
								}
							}
							drop(values_iter);

							// Then add a trailing comma if we had none
							if values.trailing.is_none() {
								values.trailing = Some(token::Comma::new());
							}
						});

						// Finally, close the indentation on the `]`
						self.0.suffix.prefix_ws_set_cur_indent(ctx);
						self.0.suffix.format(ctx);
					},
				}
			},

			Some(ArrayElements::Repeat(_)) | None => {
				self.0.format_remove(ctx);
				self.0.format(ctx);
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
	#[format(before_with = Format::prefix_ws_remove)]
	pub semi:  token::Semi,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub count: Expression,
}
