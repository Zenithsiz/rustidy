//! Struct

// Imports
use {
	super::function::{GenericParams, WhereClause},
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			delimited::{Braced, Parenthesized},
			expr::Expression,
			ident::Identifier,
			punct::{self, PunctuatedTrailing},
			token,
			ty::Type,
			vis::Visibility,
			with_attrs::{self, WithOuterAttributes},
		},
	},
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	pub inner:    StructStructInner,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructStructInner {
	#[format(and_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	Fields(Braced<Option<StructFields>>),
	#[format(and_with = Format::prefix_ws_remove)]
	Semi(token::Semi),
}

/// `StructFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFields(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_cur_indent, Format::prefix_ws_remove))]
	PunctuatedTrailing<StructField, token::Comma>,
);

/// `StructField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructField(
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<StructFieldInner>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFieldInner {
	pub vis:   Option<Visibility>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident: Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon: token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:    Type,
	// Note: Nightly-only
	#[format(and_with = Format::prefix_ws_set_single)]
	pub eq:    Option<StructFieldEq>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFieldEq {
	pub eq:   token::Eq,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
}

/// `TupleStruct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStruct {
	pub struct_:  token::Struct,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_remove)]
	#[format(and_with = Parenthesized::format_remove)]
	pub fields:   Parenthesized<Option<TupleFields>>,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(and_with = Format::prefix_ws_remove)]
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
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_single))]
	pub  WithOuterAttributes<TupleFieldInner>,
);

/// `TupleFieldInner`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleFieldInner {
	pub vis: Option<Visibility>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.vis.is_some()))]
	pub ty:  Type,
}
