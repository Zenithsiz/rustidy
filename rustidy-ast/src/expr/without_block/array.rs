//! Array

// Imports
use {
	crate::{expr::Expression, token, util::Bracketed},
	rustidy_ast_util::punct::{PunctuatedRest, PunctuatedTrailing},
	rustidy_format::{Format, Formattable, WhitespaceFormat, WsFmtFn},
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
		prefix_ws: &impl WsFmtFn,
	) {
		values.punctuated.first.format(ctx, prefix_ws, &mut ());
		for PunctuatedRest { punct: comma, value } in &mut values.punctuated.rest {
			comma.format(ctx, &Whitespace::remove, &mut ());
			value.format(ctx, &Whitespace::set_single, &mut ());
		}

		values.trailing.format(ctx, &Whitespace::remove, &mut ());
	}

	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: &impl WsFmtFn, (): &mut ()) {
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
						values.punctuated.first.format(ctx, &Whitespace::remove, &mut ());
						values.trailing = None;

						self.0.suffix.format(ctx, &Whitespace::remove, &mut ());
					},

					false => {
						ctx.with_indent(|ctx| {
							// Format the first value in each column as indentation
							let mut values_iter = values.values_mut();
							loop {
								let mut row_values = (&mut values_iter).take(cols.unwrap_or(1));
								let Some(first) = row_values.next() else { break };
								first.format(ctx, &Whitespace::set_cur_indent, &mut ());

								for value in row_values {
									value.format(ctx, &Whitespace::set_single, &mut ());
								}
							}
							drop(values_iter);

							// Then add a trailing comma if we had none
							if values.trailing.is_none() {
								values.trailing = Some(token::Comma::new());
							}
						});

						// Finally, close the indentation on the `]`
						self.0.suffix.format(ctx, &Whitespace::set_cur_indent, &mut ());
					},
				}
			},

			Some(ArrayElements::Repeat(repeat)) => {
				repeat.format(ctx, &Whitespace::remove, &mut ());
			},

			None => (),
		}
	}
}

/// `ArrayElements`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(with = |_, _, _, (): &mut ()| panic!("This type shouldn't be formatted manually"))]
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
