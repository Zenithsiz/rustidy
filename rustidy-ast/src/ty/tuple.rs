//! Tuple type

// Imports
use {
	super::Type,
	crate::{token, util::Parenthesized},
	rustidy_ast_util::delimited,
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `TupleType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a tuple type")]
pub struct TupleType(#[format(args = delimited::fmt_remove())] Parenthesized<Option<TupleTypeInner>>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
pub struct TupleTypeInner {
	pub tys: Vec<(Type, token::Comma)>,
	pub end: Option<Box<Type>>,
}

impl Format<()> for TupleTypeInner {
	fn format(
		&mut self,
		ctx: &mut rustidy_format::Context,
		prefix_ws: WhitespaceConfig,
		_args: &mut (),
	) -> FormatOutput {
		let [(first_ty, first_comma), tys @ ..] = &mut *self.tys else {
			return self.end.format(ctx, prefix_ws, &mut ());
		};

		let mut output = FormatOutput::default();

		first_ty.format(ctx, prefix_ws, &mut ()).append_to(&mut output);
		first_comma
			.format(ctx, Whitespace::REMOVE, &mut ())
			.append_to(&mut output);
		for (ty, comma) in tys {
			ty.format(ctx, Whitespace::SINGLE, &mut ()).append_to(&mut output);
			comma.format(ctx, Whitespace::REMOVE, &mut ()).append_to(&mut output);
		}
		self.end.format(ctx, Whitespace::SINGLE, &mut ()).append_to(&mut output);

		output
	}
}
