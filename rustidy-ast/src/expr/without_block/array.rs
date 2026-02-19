//! Array

// Imports
use {
	crate::{expr::Expression, token, util::Bracketed},
	rustidy_ast_util::punct::{PunctuatedRest, PunctuatedTrailing},
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
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
	) -> (FormatOutput, FormatOutput) {
		let mut common_output = FormatOutput::default();
		let mut single_line_output = FormatOutput::default();

		prefix.format(ctx, prefix_ws, ()).append_to(&mut common_output);
		values
			.punctuated
			.first
			.format(ctx, Whitespace::REMOVE, ())
			.append_to(&mut common_output);

		for PunctuatedRest { punct: comma, value } in &mut values.punctuated.rest {
			comma.format(ctx, Whitespace::REMOVE, ()).append_to(&mut common_output);
			value
				.format(ctx, Whitespace::SINGLE, ())
				.append_to(&mut single_line_output);
		}

		// TODO: Should we preserve the original comma by returning it?
		values.trailing = None;
		suffix
			.format(ctx, Whitespace::REMOVE, ())
			.append_to(&mut single_line_output);

		(common_output, single_line_output)
	}
}

impl Format<()> for ArrayExpression {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: WhitespaceConfig, _args: ()) -> FormatOutput {
		match &mut self.0.value {
			Some(ArrayElements::Punctuated(values)) => {
				let (common_output, single_line_output) =
					Self::format_single_line(&mut self.0.prefix, values, &mut self.0.suffix, ctx, prefix_ws);
				let single_line_output = FormatOutput::join(common_output, single_line_output);

				// Then check if we can fit into a single line.
				// Note: If the user specified a number of columns, only
				//       put everything into a single line if they fit.
				// TODO: Should we even special case that?
				let cols = ctx.config().array_expr_cols;
				let is_single_line = !single_line_output.has_newlines &&
					match cols {
						Some(cols) => cols >= values.values_len(),
						None => single_line_output.len_without_prefix_ws() <= ctx.config().max_array_expr_len,
					};

				// If we don't fit in a single line, format it multi-line
				match is_single_line {
					true => single_line_output,
					false => {
						let mut output = common_output;

						ctx.with_indent(|ctx| {
							// Format the first value in each column as indentation
							let mut values_iter = values.values_mut();
							loop {
								let mut row_values = (&mut values_iter).take(cols.unwrap_or(1));
								let Some(first) = row_values.next() else { break };
								first.format(ctx, Whitespace::CUR_INDENT, ()).append_to(&mut output);

								for value in row_values {
									value.format(ctx, Whitespace::SINGLE, ()).append_to(&mut output);
								}
							}
							drop(values_iter);

							if values.trailing.is_none() {
								values.trailing = Some(token::Comma::new());
							}
							values
								.trailing
								.format(ctx, Whitespace::REMOVE, ())
								.append_to(&mut output);
						});

						// Finally, close the indentation on the `]`
						self.0
							.suffix
							.format(ctx, Whitespace::CUR_INDENT, ())
							.append_to(&mut output);

						output
					},
				}
			},

			Some(ArrayElements::Repeat(repeat)) => {
				let mut output = FormatOutput::default();

				self.0.prefix.format(ctx, prefix_ws, ()).append_to(&mut output);
				repeat.format(ctx, Whitespace::REMOVE, ()).append_to(&mut output);
				self.0.suffix.format(ctx, Whitespace::REMOVE, ()).append_to(&mut output);

				output
			},

			None => FormatOutput::default(),
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
