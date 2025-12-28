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
			ident::Identifier,
			punct::PunctuatedTrailing,
			token,
			ty::Type,
			vis::Visibility,
			with_attrs::WithOuterAttributes,
		},
	},
};

/// `Struct`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum Struct {
	Struct(StructStruct),
	Tuple(TupleStruct),
}

/// `StructStruct`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructStruct {
	pub struct_:  token::Struct,
	pub ident:    Identifier,
	pub generics: Option<GenericParams>,
	pub where_:   Option<WhereClause>,
	pub inner:    StructStructInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum StructStructInner {
	Fields(Braced<Option<StructFields>>),
	Semi(token::Semi),
}

/// `StructFields`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFields(PunctuatedTrailing<StructField, token::Comma>);

/// `StructField`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructField(pub WithOuterAttributes<StructFieldInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StructFieldInner {
	pub vis:   Option<Visibility>,
	pub ident: Identifier,
	pub colon: token::Colon,
	pub ty:    Type,
}

/// `TupleStruct`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStruct {
	pub struct_:  token::Struct,
	pub ident:    Identifier,
	pub generics: Option<GenericParams>,
	pub fields:   Parenthesized<Option<TupleFields>>,
	#[parse(fatal)]
	pub where_:   Option<WhereClause>,
	pub semi:     token::Semi,
}

/// `TupleFields`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleFields(PunctuatedTrailing<TupleField, token::Comma>);

/// `TupleField`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleField(pub WithOuterAttributes<TupleFieldInner>);

/// `TupleFieldInner`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleFieldInner {
	pub vis: Option<Visibility>,
	pub ty:  Type,
}
