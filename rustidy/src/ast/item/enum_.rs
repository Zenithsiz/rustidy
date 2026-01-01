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
	pub vis:          Option<Visibility>,
	pub ident:        Identifier,
	pub kind:         Option<EnumVariantKind>,
	pub discriminant: Option<EnumVariantDiscriminant>,
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
pub struct EnumVariantTuple(pub Parenthesized<Option<TupleFields>>);

/// `EnumVariantStruct`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantStruct(pub Braced<Option<StructFields>>);

/// `EnumVariantDiscriminant`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantDiscriminant {
	pub eq:   token::Eq,
	pub expr: Expression,
}
