//! Tuple type

// Imports
use {
	super::Type,
	crate::{token, util::Parenthesized},
	rustidy_ast_util::delimited,
	rustidy_format::{Format, WhitespaceConfig, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `TupleType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a tuple type")]
pub struct TupleType(#[format(args = delimited::fmt_remove())] Parenthesized<Option<TupleTypeInner>>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(skip_format)]
pub struct TupleTypeInner {
	pub tys: Vec<(Type, token::Comma)>,
	pub end: Option<Box<Type>>,
}

impl Format<()> for TupleTypeInner {
	fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: WhitespaceConfig, _args: &mut ()) {
		let [(first_ty, first_comma), tys @ ..] = &mut *self.tys else {
			self.end.format(ctx, prefix_ws, &mut ());
			return;
		};

		first_ty.format(ctx, prefix_ws, &mut ());
		first_comma.format(ctx, Whitespace::REMOVE, &mut ());
		for (ty, comma) in tys {
			ty.format(ctx, Whitespace::SINGLE, &mut ());
			comma.format(ctx, Whitespace::REMOVE, &mut ());
		}
		self.end.format(ctx, Whitespace::SINGLE, &mut ());
	}
}
