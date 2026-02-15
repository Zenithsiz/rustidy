//! Enums

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		struct_::{StructFields, TupleFields},
	},
	crate::{
		attr::WithOuterAttributes,
		expr::Expression,
		token,
		util::{Braced, Parenthesized},
		vis::Visibility,
	},
	rustidy_ast_util::{Identifier, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Enumeration`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct Enumeration {
	pub enum_:    token::Enum,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generic:  Option<GenericParams>,
	#[format(prefix_ws = Whitespace::CUR_INDENT)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(indent)]
	#[format(args = delimited::FmtArgs::indent_if_non_blank((), (), ()))]
	pub variants: Braced<Option<EnumVariants>>,
}

/// `EnumVariants`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariants(
	#[format(args = punct::FmtArgs::new(Whitespace::CUR_INDENT, Whitespace::REMOVE))]
	pub  PunctuatedTrailing<EnumVariant, token::Comma>,
);

/// `EnumVariant`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariant(pub WithOuterAttributes<EnumVariantInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantInner {
	pub vis:          Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.vis.is_some()))]
	pub ident:        Identifier,
	pub kind:         Option<EnumVariantKind>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub discriminant: Option<EnumVariantDiscriminant>,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum EnumVariantKind {
	#[format(prefix_ws = Whitespace::REMOVE)]
	Tuple(EnumVariantTuple),
	#[format(prefix_ws = Whitespace::SINGLE)]
	Struct(EnumVariantStruct),
}

/// `EnumVariantTuple`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantTuple(
	#[format(args = delimited::FmtArgs::remove((), (), ()))] pub Parenthesized<Option<TupleFields>>,
);

/// `EnumVariantStruct`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantStruct(
	#[format(indent)]
	#[format(args = delimited::FmtArgs::indent_if_non_blank((), (), ()))]
	pub Braced<Option<StructFields>>,
);

/// `EnumVariantDiscriminant`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct EnumVariantDiscriminant {
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}
