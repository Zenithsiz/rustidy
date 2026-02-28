//! Array

// Imports
use {
	crate::{expr::Expression, util::{Bracketed, FmtRemoveOrIndent}},

	ast_util::{delimited, punct::{self, PunctuatedTrailing}},
	format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	parse::Parse,
	print::Print,
	util::Whitespace,
};

/// `ArrayExpression`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "an array expression")]
pub struct ArrayExpression(
	#[format(args = delimited::FmtArgsRemoveOrIndentIfNonBlank {
		// TODO: Maybe allow multiline as long as it's the only/last?
		force_indent_on_multiline: true,

		// Note: If the user specified columns and we can't fit everything into a single
		//       column, then ensure we do multi-line by setting the max length to 0.
		max_len: match ctx.config().array_expr_cols {
			Some(cols) if let Some(ArrayElements::Punctuated(values)) = &self.0.value && values.values_len() > cols => 0,
			_ => ctx.config().max_array_expr_len,
		},

		value_args_remove: FmtRemoveOrIndent::Remove,
		value_args_indent: FmtRemoveOrIndent::Indent
	})]
	Bracketed<Option<ArrayElements>>,
);

/// `ArrayElements`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "FmtRemoveOrIndent"))]
pub enum ArrayElements {
	Repeat(ArrayElementsRepeat),
	#[format(with = Self::format_punctuated)]
	#[format(args = args)]
	Punctuated(PunctuatedTrailing<Expression, ast_token::Comma>),
}

impl ArrayElements {
	pub fn format_punctuated(
		values: &mut PunctuatedTrailing<Expression, ast_token::Comma>,
		ctx: &mut format::Context,
		prefix_ws: WhitespaceConfig,
		args: FmtRemoveOrIndent
	) -> FormatOutput {
		match args {
			FmtRemoveOrIndent::Remove => ctx.format_with(
				values,
				prefix_ws,
				punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE)
			),
			FmtRemoveOrIndent::Indent => ctx.format_with(
				values,
				prefix_ws,
				punct::FmtIndentColumns { columns: ctx.config().array_expr_cols, }
			),
		}
	}
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct ArrayElementsRepeat {
	pub expr:  Expression,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:  ast_token::Semi,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub count: Expression,
}
