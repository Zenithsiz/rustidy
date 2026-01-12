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
			punct::{self, PunctuatedTrailing},
			token,
			vis::Visibility,
			with_attrs::{self, WithOuterAttributes},
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
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generic:  Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(and_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub variants: Braced<Option<EnumVariants>>,
}

/// `EnumVariants`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariants(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_cur_indent, Format::prefix_ws_remove))]
	pub  PunctuatedTrailing<EnumVariant, token::Comma>,
);

/// `EnumVariant`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariant(
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<EnumVariantInner>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantInner {
	pub vis:          Option<Visibility>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:        Identifier,
	pub kind:         Option<EnumVariantKind>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub discriminant: Option<EnumVariantDiscriminant>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum EnumVariantKind {
	#[format(and_with = Format::prefix_ws_remove)]
	Tuple(EnumVariantTuple),
	#[format(and_with = Format::prefix_ws_set_single)]
	Struct(EnumVariantStruct),
}

/// `EnumVariantTuple`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantTuple(#[format(and_with = Parenthesized::format_remove)] pub Parenthesized<Option<TupleFields>>);

/// `EnumVariantStruct`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantStruct(
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub Braced<Option<StructFields>>,
);

/// `EnumVariantDiscriminant`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantDiscriminant {
	pub eq:   token::Eq,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
}
