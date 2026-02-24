//! Array

// Imports
use {
	crate::{expr::Expression, token, util::Bracketed},
	rustidy_ast_util::{Delimited, delimited, punct::{self, PunctuatedTrailing}},
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `ArrayExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
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
	) -> FormatOutput {
		let mut delimited = Delimited { prefix, value: values, suffix, };
		let args = delimited::FmtRemoveWith(
			punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE)
		);
		delimited.format(ctx, prefix_ws, args)
	}

	/// Formats all `values` as multi-line
	fn format_multi_line(
		prefix: &mut token::BracketOpen,
		values: &mut PunctuatedTrailing<Expression, token::Comma>,
		suffix: &mut token::BracketClose,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
	) -> FormatOutput {
		let mut delimited = Delimited { prefix, value: values, suffix, };
		let args = delimited::fmt_indent_if_non_blank_with_value(
			punct::FmtIndentColumns { columns: ctx.config().array_expr_cols, }
		);
		delimited.format(ctx, prefix_ws, args)
	}
}

impl Format<WhitespaceConfig, ()> for ArrayExpression {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		_args: ()
	) -> FormatOutput {
		match &mut self.0.value {
			Some(ArrayElements::Punctuated(values)) => {
				let trailing_comma = values.trailing.take();
				let single_line_output = Self::format_single_line(
					&mut self.0.prefix,
					values,
					&mut self.0.suffix,
					ctx,
					prefix_ws
				);

				// Then check if we can fit into a single line.
				// Note: If the user specified a number of columns, only
				//       put everything into a single line if they fit.
				// TODO: Should we even special case that?
				let cols = ctx.config().array_expr_cols;
				let is_single_line = !single_line_output.has_newlines() && match cols {
					Some(cols) => cols >= values.values_len(),
					None => single_line_output.len_without_prefix_ws() <= ctx.config().max_array_expr_len,
				};

				// If we don't fit in a single line, format it multi-line
				match is_single_line {
					true => single_line_output,
					false => {
						values.trailing = Some(trailing_comma.unwrap_or_default());
						Self::format_multi_line(
							&mut self.0.prefix,
							values,
							&mut self.0.suffix,
							ctx,
							prefix_ws,
						)
					},
				}
			},

			Some(ArrayElements::Repeat(repeat)) => {
				let mut output = FormatOutput::default();

				ctx
					.format(&mut self.0.prefix, prefix_ws)
					.append_to(&mut output);
				ctx
					.format(repeat, Whitespace::REMOVE)
					.append_to(&mut output);
				ctx
					.format(&mut self.0.suffix, Whitespace::REMOVE)
					.append_to(&mut output);

				output
			},

			None => FormatOutput::default(),
		}
	}
}

/// `ArrayElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
pub enum ArrayElements {
	Repeat(ArrayElementsRepeat),
	Punctuated(PunctuatedTrailing<Expression, token::Comma>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ArrayElementsRepeat {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:  token::Semi,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub count: Expression,
}
