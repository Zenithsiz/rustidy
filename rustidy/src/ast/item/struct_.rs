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
	struct_:  token::Struct,
	ident:    Identifier,
	generics: Option<GenericParams>,
	where_:   Option<WhereClause>,
	inner:    StructStructInner,
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
	vis:   Option<Visibility>,
	ident: Identifier,
	colon: token::Colon,
	ty:    Type,
}

/// `TupleStruct`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TupleStruct {
	struct_:  token::Struct,
	ident:    Identifier,
	generics: Option<GenericParams>,
	fields:   Parenthesized<Option<TupleFields>>,
	#[parse(fatal)]
	where_:   Option<WhereClause>,
	semi:     token::Semi,
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
	vis: Option<Visibility>,
	ty:  Type,
}
