//! Tuple type

// Imports
use {
	super::Type,
	crate::{token, util::Parenthesized},
	rustidy_ast_util::delimited,
	rustidy_format::{Format, WhitespaceFormat, WsFmtFn},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `TupleType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a tuple type")]
pub struct TupleType(#[format(args = delimited::FmtArgs::remove((), (), ()))] Parenthesized<Option<TupleTypeInner>>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(with = Self::format_tys)]
pub struct TupleTypeInner {
	pub tys: Vec<(Type, token::Comma)>,
	pub end: Option<Box<Type>>,
}

impl TupleTypeInner {
	fn format_tys(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: &mut impl WsFmtFn, _args: &mut ()) {
		let [(first_ty, first_comma), tys @ ..] = &mut *self.tys else {
			self.end.format(ctx, prefix_ws, &mut ());
			return;
		};

		first_ty.format(ctx, prefix_ws, &mut ());
		first_comma.format(ctx, &mut Whitespace::remove, &mut ());
		for (ty, comma) in tys {
			ty.format(ctx, &mut Whitespace::set_single, &mut ());
			comma.format(ctx, &mut Whitespace::remove, &mut ());
		}
		self.end.format(ctx, &mut Whitespace::set_single, &mut ());
	}
}
