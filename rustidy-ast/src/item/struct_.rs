//! Struct

// Imports
use {
	super::function::{GenericParams, WhereClause},
	crate::{
		attr::{self, WithOuterAttributes},
		expr::Expression,
		token,
		ty::Type,
		util::{Braced, Parenthesized},
		vis::Visibility,
	},
	rustidy_ast_util::{Identifier, PunctuatedTrailing, punct},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Struct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum Struct {
	Struct(StructStruct),
	Tuple(TupleStruct),
}

/// `StructStruct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructStruct {
	pub struct_:  token::Struct,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(before_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	pub inner:    StructStructInner,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructStructInner {
	#[format(before_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	Fields(Braced<Option<StructFields>>),
	#[format(before_with = Format::prefix_ws_remove)]
	Semi(token::Semi),
}

/// `StructFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(and_with = Self::align_fields)]
pub struct StructFields(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_cur_indent, Format::prefix_ws_remove))]
	PunctuatedTrailing<StructField, token::Comma>,
);

impl StructFields {
	/// Aligns all fields
	pub fn align_fields(&mut self, ctx: &mut rustidy_format::Context) {
		let Some(max_ident_len) = self.0.values().map(|field| field.0.inner.ident.non_ws_len()).max() else {
			return;
		};

		for field in self.0.values_mut() {
			let ident_len = field.0.inner.ident.non_ws_len();
			let ty_prefix_ws_len = 1 + max_ident_len - ident_len;
			field.0.inner.ty.prefix_ws_set_spaces(ctx, ty_prefix_ws_len);
		}
	}
}

/// `StructField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructField(
	#[format(and_with = attr::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<StructFieldInner>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFieldInner {
	pub vis:   Option<Visibility>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.vis.is_some()))]
	pub ident: Identifier,
	#[format(before_with = Format::prefix_ws_remove)]
	pub colon: token::Colon,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ty:    Type,
	// Note: Nightly-only
	#[format(before_with = Format::prefix_ws_set_single)]
	pub eq:    Option<StructFieldEq>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFieldEq {
	pub eq:   token::Eq,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
}

/// `TupleStruct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStruct {
	pub struct_:  token::Struct,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(before_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(before_with = Format::prefix_ws_remove)]
	#[format(and_with = Parenthesized::format_remove)]
	pub fields:   Parenthesized<Option<TupleFields>>,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(before_with = Format::prefix_ws_remove)]
	pub semi:     token::Semi,
}

/// `TupleFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleFields(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_single, Format::prefix_ws_remove))]
	PunctuatedTrailing<TupleField, token::Comma>,
);

/// `TupleField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleField(
	#[format(and_with = attr::format_outer_value_non_empty(Format::prefix_ws_set_single))]
	pub  WithOuterAttributes<TupleFieldInner>,
);

/// `TupleFieldInner`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleFieldInner {
	pub vis: Option<Visibility>,
	#[format(before_with(expr = Format::prefix_ws_set_single, if = self.vis.is_some()))]
	pub ty:  Type,
}
