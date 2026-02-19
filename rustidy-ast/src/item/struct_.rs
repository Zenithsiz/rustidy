//! Struct

// Imports
use {
	super::function::{GenericParams, WhereClause},
	crate::{
		attr::{WithOuterAttributes, with},
		expr::Expression,
		token,
		ty::Type,
		util::{Braced, Parenthesized},
		vis::Visibility,
	},
	rustidy_ast_util::{Identifier, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Struct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum Struct {
	Struct(StructStruct),
	Tuple(TupleStruct),
}

/// `StructStruct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructStruct {
	pub struct_:  token::Struct,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::CUR_INDENT)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = match self.inner {
		StructStructInner::Fields(_) => Whitespace::SINGLE,
		StructStructInner::Semi(_) => Whitespace::REMOVE,
	})]
	pub inner:    StructStructInner,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(strum::EnumIs)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum StructStructInner {
	#[format(indent)]
	#[format(args = delimited::fmt_indent_if_non_blank())]
	Fields(Braced<Option<StructFields>>),
	Semi(token::Semi),
}

/// `StructFields`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructFields(
	#[format(args = {
		let max_ident_len = self.0.values().map(|field| field.0.inner.ident.non_ws_len()).max().expect("At least one element exists");
		punct::fmt_with(Whitespace::CUR_INDENT, Whitespace::REMOVE, StructFieldInnerArgs { max_ident_len }, ())
	})]
	PunctuatedTrailing<StructField, token::Comma>,
);

/// `StructField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "StructFieldInnerArgs"))]
pub struct StructField(#[format(args = with::fmt(args))] pub WithOuterAttributes<StructFieldInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[format(args(ty = "StructFieldInnerArgs"))]
pub struct StructFieldInner {
	pub vis:   Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.vis.is_some()))]
	pub ident: Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub colon: token::Colon,
	#[format(prefix_ws = {
		let ident_len = self.ident.non_ws_len();
		let ty_prefix_ws_len = 1 + args.max_ident_len - ident_len;
		Whitespace::spaces(ty_prefix_ws_len)
	})]
	pub ty:    Type,
	// Note: Nightly-only
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub eq:    Option<StructFieldEq>,
}

#[derive(Clone, Copy, Debug)]
struct StructFieldInnerArgs {
	max_ident_len: usize,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct StructFieldEq {
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}

/// `TupleStruct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
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
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleFields(
	#[format(args = punct::fmt(Whitespace::SINGLE, Whitespace::REMOVE))] PunctuatedTrailing<TupleField, token::Comma>,
);

/// `TupleField`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleField(pub WithOuterAttributes<TupleFieldInner>);

/// `TupleFieldInner`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TupleFieldInner {
	pub vis: Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.vis.is_some()))]
	pub ty:  Type,
}
