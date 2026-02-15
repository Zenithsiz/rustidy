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
	crate::{
		attr::{BracedWithInnerAttributes, WithOuterAttributes},
		token,
	},
	rustidy_ast_util::Identifier,
	rustidy_format::{Format, WhitespaceFormat},
	rustidy_parse::Parse,
	rustidy_print::Print,
	rustidy_util::Whitespace,
};

/// `Trait`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a trait")]
pub struct Trait {
	pub unsafe_:  Option<token::Unsafe>,
	// Note: Nightly-only
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.unsafe_.is_some()))]
	pub auto:     Option<token::Auto>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.unsafe_.is_some() || self.auto.is_some()))]
	pub trait_:   token::Trait,
	#[parse(fatal)]
	#[format(prefix_ws = Whitespace::set_single)]
	pub ident:    Identifier,
	#[format(prefix_ws = Whitespace::remove)]
	pub generics: Option<GenericParams>,
	#[format(prefix_ws = Whitespace::remove)]
	pub bounds:   Option<TraitColonBounds>,
	#[format(prefix_ws = Whitespace::set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(prefix_ws = Whitespace::set_single)]
	pub body:     TraitBody,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TraitBody {
	// Note: Nightly-only
	Eq(TraitBodyEq),
	Full(BracedWithInnerAttributes<TraitBodyFull>),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBodyEq {
	pub eq:     token::Eq,
	#[format(prefix_ws = Whitespace::set_single)]
	pub bounds: Option<TypeParamBounds>,
	#[format(prefix_ws = Whitespace::remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBodyFull(
	#[format(args = rustidy_format::vec::Args::from_prefix_ws(Whitespace::set_cur_indent))] pub Vec<AssociatedItem>,
);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitColonBounds {
	pub colon:  token::Colon,
	#[format(prefix_ws = Whitespace::set_single)]
	pub bounds: Option<TypeParamBounds>,
}

/// `AssociatedItem`
#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AssociatedItem(pub WithOuterAttributes<AssociatedItemInner>);

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AssociatedItemInner {
	Macro(MacroInvocationSemi),
	Vis(AssociatedItemVis),
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AssociatedItemVis {
	pub vis:   Option<Visibility>,
	#[format(prefix_ws(expr = Whitespace::set_single, if = self.vis.is_some()))]
	pub inner: AssociatedItemVisInner,
}

#[derive(PartialEq, Eq, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AssociatedItemVisInner {
	TypeAlias(TypeAlias),
	Constant(ConstantItem),
	Function(Function),
}
