//! Tuple type

// Imports
use {
	super::Type,
	crate::ast::{delimited::Parenthesized, token},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `TupleType`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a tuple type")]
pub struct TupleType(#[format(and_with = Parenthesized::format_remove)] Parenthesized<Option<TupleTypeInner>>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(and_with = Self::format_tys)]
pub struct TupleTypeInner {
	pub tys: Vec<(Type, token::Comma)>,
	pub end: Option<Box<Type>>,
}

impl TupleTypeInner {
	fn format_tys(&mut self, ctx: &mut rustidy_format::Context) {
		let Some(((_, first_comma), tys)) = self.tys.split_first_mut() else {
			return;
		};

		first_comma.prefix_ws_remove(ctx);
		for (ty, comma) in tys {
			ty.prefix_ws_set_single(ctx);
			comma.prefix_ws_remove(ctx);
		}
		if let Some(end_ty) = &mut self.end {
			end_ty.prefix_ws_set_single(ctx);
		}
	}
}
