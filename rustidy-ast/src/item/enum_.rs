//! Enums

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		struct_::{StructFields, TupleFields},
	},
	crate::{
		attr::{self, WithOuterAttributes},
		expr::Expression,
		token,
		util::{Braced, Parenthesized},
		vis::Visibility,
	},
	rustidy_ast_util::{Identifier, PunctuatedTrailing, punct},
	rustidy_format::Format,
	rustidy_parse::Parse,
	rustidy_print::Print,
};

/// `Enumeration`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Enumeration {
	pub enum_:    token::Enum,
	#[parse(fatal)]
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(before_with = Format::prefix_ws_remove)]
	pub generic:  Option<GenericParams>,
	#[format(before_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(before_with = Format::prefix_ws_set_single)]
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub variants: Braced<Option<EnumVariants>>,
}

/// `EnumVariants`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariants(
	#[format(and_with = punct::format_trailing(Format::prefix_ws_set_cur_indent, Format::prefix_ws_remove))]
	pub  PunctuatedTrailing<EnumVariant, token::Comma>,
);

/// `EnumVariant`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariant(
	#[format(and_with = attr::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<EnumVariantInner>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantInner {
	pub vis:          Option<Visibility>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub ident:        Identifier,
	pub kind:         Option<EnumVariantKind>,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub discriminant: Option<EnumVariantDiscriminant>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum EnumVariantKind {
	#[format(before_with = Format::prefix_ws_remove)]
	Tuple(EnumVariantTuple),
	#[format(before_with = Format::prefix_ws_set_single)]
	Struct(EnumVariantStruct),
}

/// `EnumVariantTuple`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantTuple(#[format(and_with = Parenthesized::format_remove)] pub Parenthesized<Option<TupleFields>>);

/// `EnumVariantStruct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantStruct(
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_blank)]
	pub Braced<Option<StructFields>>,
);

/// `EnumVariantDiscriminant`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantDiscriminant {
	pub eq:   token::Eq,
	#[format(before_with = Format::prefix_ws_set_single)]
	pub expr: Expression,
}
