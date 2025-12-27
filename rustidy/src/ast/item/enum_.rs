//! Enums

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		struct_::{StructFields, TupleFields},
	},
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			delimited::{Braced, Parenthesized},
			expr::Expression,
			ident::Identifier,
			punct::PunctuatedTrailing,
			token,
			vis::Visibility,
			with_attrs::WithOuterAttributes,
		},
	},
};

/// `Enumeration`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Enumeration {
	pub enum_:    token::Enum,
	#[parse(fatal)]
	pub ident:    Identifier,
	pub generic:  Option<GenericParams>,
	pub where_:   Option<WhereClause>,
	pub variants: Braced<Option<EnumVariants>>,
}

/// `EnumVariants`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariants(PunctuatedTrailing<EnumVariant, token::Comma>);

/// `EnumVariant`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariant(pub WithOuterAttributes<EnumVariantInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantInner {
	vis:          Option<Visibility>,
	ident:        Identifier,
	kind:         Option<EnumVariantKind>,
	discriminant: Option<EnumVariantDiscriminant>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum EnumVariantKind {
	Tuple(EnumVariantTuple),
	Struct(EnumVariantStruct),
}

/// `EnumVariantTuple`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantTuple(pub Parenthesized<TupleFields>);

/// `EnumVariantStruct`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantStruct(pub Braced<StructFields>);

/// `EnumVariantDiscriminant`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantDiscriminant {
	eq:   token::Eq,
	expr: Expression,
}
