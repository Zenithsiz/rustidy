//! Enums

// Imports
use {
	super::{
		function::{GenericParams, WhereClause},
		struct_::{StructFields, TupleFields, TupleStruct},
	},
	crate::{
		attr::WithOuterAttributes,
		expr::Expression,
		token,
		util::{Braced, Parenthesized},
		vis::Visibility,
	},
	rustidy_ast_util::{Identifier, PunctuatedTrailing, delimited, punct},
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Enumeration`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct Enumeration {
	pub enum_:    token::Enum,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generic:  Option<GenericParams>,
	#[format(prefix_ws = Whitespace::INDENT)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	#[format(indent)]
	#[format(args = delimited::fmt_indent_if_non_blank())]
	pub variants: Braced<Option<EnumVariants>>,
}

/// `EnumVariants`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct EnumVariants(
	#[format(args = punct::fmt(Whitespace::INDENT, Whitespace::REMOVE))]
	pub PunctuatedTrailing<EnumVariant, token::Comma>,
);

/// `EnumVariant`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct EnumVariant(pub WithOuterAttributes<EnumVariantInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct EnumVariantInner {
	pub vis:          Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.vis.is_some()))]
	pub ident:        Identifier,
	#[format(prefix_ws(if = let Some(kind) = &self.kind, expr = match kind {
		EnumVariantKind::Tuple(_) => Whitespace::REMOVE,
		EnumVariantKind::Struct(_) => Whitespace::SINGLE,
	}))]
	pub kind:         Option<EnumVariantKind>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub discriminant: Option<EnumVariantDiscriminant>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum EnumVariantKind {
	Tuple(EnumVariantTuple),
	Struct(EnumVariantStruct),
}

/// `EnumVariantTuple`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct EnumVariantTuple(
	#[format(with = TupleStruct::format_fields)]
	pub Parenthesized<Option<TupleFields>>,
);

/// `EnumVariantStruct`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct EnumVariantStruct(
	#[format(indent)]
	// TODO: Use `delimited::fmt_single_or_indent_if_non_blank`
	#[format(args = delimited::fmt_indent_if_non_blank())]
	pub Braced<Option<StructFields>>,
);

/// `EnumVariantDiscriminant`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct EnumVariantDiscriminant {
	pub eq:   token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub expr: Expression,
}
