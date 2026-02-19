//! Trait statement

// Imports
use {
	super::{
		ConstantItem,
		Function,
		MacroInvocationSemi,
		TypeAlias,
		Visibility,
		function::{GenericParams, TypeParamBounds, WhereClause},
	},
	crate::{attr::{BracedWithInnerAttributes, WithOuterAttributes}, token},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, Formattable, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Trait`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
#[parse(name = "a trait")]
pub struct Trait {
	pub unsafe_:  Option<token::Unsafe>,
	// Note: Nightly-only
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.unsafe_.is_some()))]
	pub auto:     Option<token::Auto>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.unsafe_.is_some() || self.auto.is_some()))]
	pub trait_:   token::Trait,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub bounds:   Option<TraitColonBounds>,
	#[format(prefix_ws = Whitespace::CUR_INDENT)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub body:     TraitBody,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum TraitBody {
	// Note: Nightly-only
	Eq(TraitBodyEq),
	Full(BracedWithInnerAttributes<TraitBodyFull>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TraitBodyEq {
	pub eq:     token::Eq,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bounds: Option<TypeParamBounds>,
	#[format(prefix_ws = Whitespace::REMOVE)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TraitBodyFull(
	#[format(args = rustidy_format::vec::args_prefix_ws(Whitespace::CUR_INDENT))]
	pub Vec<AssociatedItem>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct TraitColonBounds {
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::SINGLE)]
	pub bounds: Option<TypeParamBounds>,
}

/// `AssociatedItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct AssociatedItem(pub WithOuterAttributes<AssociatedItemInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum AssociatedItemInner {
	Macro(MacroInvocationSemi),
	Vis(AssociatedItemVis),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub struct AssociatedItemVis {
	pub vis:   Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::SINGLE, if = self.vis.is_some()))]
	pub inner: AssociatedItemVisInner,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Formattable, Format, Print)]
pub enum AssociatedItemVisInner {
	TypeAlias(TypeAlias),
	Constant(ConstantItem),
	Function(Function),
}
