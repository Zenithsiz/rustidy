//! Struct

// Imports
use {
	super::function::{GenericParams, WhereClause},
	crate::{
		attr::WithOuterAttributes,
		expr::Expression,
		token,
		ty::Type,
		util::{Braced, Parenthesized},
		vis::Visibility,
	},
	rustidy_ast_util::{Identifier, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
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
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::CUR_INDENT)]
	pub where_:   Option<WhereClause>,
	pub inner:    StructStructInner,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructStructInner {
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(indent)]
	#[format(args = delimited::fmt_indent_if_non_blank())]
	Fields(Braced<Option<StructFields>>),
	#[format(prefix_ws = Whitespace::REMOVE)]
	Semi(token::Semi),
}

/// `StructFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[format(and_with = Self::align_fields)]
pub struct StructFields(
	#[format(args = punct::fmt(Whitespace::CUR_INDENT, Whitespace::REMOVE))]
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
			let prefix_ws = Whitespace::spaces(ty_prefix_ws_len);
			field.0.inner.ty.format(ctx, prefix_ws, &mut ());
		}
	}
}

/// `StructField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructField(pub WithOuterAttributes<StructFieldInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFieldInner {
	pub vis:   Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.vis.is_some()))]
	pub ident: Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon: token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ty:    Type,
	// Note: Nightly-only
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:    Option<StructFieldEq>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFieldEq {
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}

/// `TupleStruct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStruct {
	pub struct_:  token::Struct,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	#[format(args = delimited::fmt_remove())]
	pub fields:   Parenthesized<Option<TupleFields>>,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::CUR_INDENT)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:     token::Semi,
}

/// `TupleFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleFields(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))] PunctuatedTrailing<TupleField, token::Comma>,
);

/// `TupleField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleField(pub WithOuterAttributes<TupleFieldInner>);

/// `TupleFieldInner`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleFieldInner {
	pub vis: Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.vis.is_some()))]
	pub ty:  Type,
}
