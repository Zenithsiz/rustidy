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
	fn format(&mut self, ctx: &mut rustidy_format::Context) {
		match &mut self.0.value {
			Some(ArrayElements::Punctuated(values)) => {
				ctx.with_indent(|ctx| {
					let cols = ctx.config().array_expr_cols;

					for comma in values.puncts_mut() {
						comma.prefix_ws_remove(ctx);
					}

					let mut values = values.values_mut();
					while let Some(first) = values.next() {
						first.prefix_ws_set_cur_indent(ctx);
						first.format(ctx);

						for _ in 1..cols.unwrap_or(1) {
							let Some(value) = values.next() else { break };
							value.prefix_ws_set_single(ctx);
							value.format(ctx);
						}
					}
				});

				self.0.suffix.prefix_ws_set_cur_indent(ctx);
				self.0.suffix.format(ctx);
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
