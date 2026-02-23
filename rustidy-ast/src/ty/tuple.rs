//! Tuple type

// Imports
use {
	crate::{token, util::Parenthesized},
	super::Type,
	rustidy_ast_util::delimited,
	rustidy_format::{Format, FormatOutput, Formattable, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `TupleType`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a tuple type")]
pub struct TupleType(#[format(args = delimited::FmtRemove)]
Parenthesized<Option<TupleTypeInner>>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Print)]
pub struct TupleTypeInner {
	pub tys: Vec<(Type, token::Comma)>,
	pub end: Option<Box<Type>>,
}

impl Format<WhitespaceConfig, ()> for TupleTypeInner {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: WhitespaceConfig, _args: ()) -> FormatOutput {
		let [(first_ty, first_comma), tys @ ..] = &mut *self.tys else {
			return ctx.format(&mut self.end, prefix_ws);
		};

		let mut output = FormatOutput::default();

		ctx
			.format(first_ty, prefix_ws)
			.append_to(&mut output);
		ctx
			.format(first_comma, Whitespace::REMOVE)
			.append_to(&mut output);
		for (ty, comma) in tys {
			ctx
				.format(ty, Whitespace::SINGLE)
				.append_to(&mut output);
			ctx
				.format(comma, Whitespace::REMOVE)
				.append_to(&mut output);
		}
		ctx
			.format(&mut self.end, Whitespace::SINGLE)
			.append_to(&mut output);

		output
	}
}
