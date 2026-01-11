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
		Format,
		ast::{
			delimited::Braced,
			ident::Identifier,
			token,
			with_attrs::{self, WithInnerAttributes, WithOuterAttributes},
		},
		format,
		parser::Parse,
		print::Print,
	},
};

/// `Trait`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "a trait")]
pub struct Trait {
	pub unsafe_:  Option<token::Unsafe>,
	// Note: Nightly-only
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.unsafe_.is_some()))]
	pub auto:     Option<token::Auto>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.unsafe_.is_some() || self.auto.is_some()))]
	pub trait_:   token::Trait,
	#[parse(fatal)]
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:    Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub generics: Option<GenericParams>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub bounds:   Option<TraitColonBounds>,
	#[format(and_with = Format::prefix_ws_set_cur_indent)]
	pub where_:   Option<WhereClause>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub body:     TraitBody,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TraitBody {
	// Note: Nightly-only
	Eq(TraitBodyEq),
	#[format(indent)]
	#[format(and_with = Braced::format_indent_if_non_empty)]
	Full(Braced<WithInnerAttributes<TraitBodyFull>>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBodyEq {
	pub eq:     token::Eq,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bounds: Option<TypeParamBounds>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBodyFull(
	#[format(and_with = format::format_vec_each_with_all(Format::prefix_ws_set_cur_indent))] pub Vec<AssociatedItem>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitColonBounds {
	pub colon:  token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub bounds: Option<TypeParamBounds>,
}

/// `AssociatedItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AssociatedItem(
	#[format(and_with = with_attrs::format_outer_value_non_empty(Format::prefix_ws_set_cur_indent))]
	pub  WithOuterAttributes<AssociatedItemInner>,
);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AssociatedItemInner {
	Macro(MacroInvocationSemi),
	Vis(AssociatedItemVis),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AssociatedItemVis {
	pub vis:   Option<Visibility>,
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.vis.is_some()))]
	pub inner: AssociatedItemVisInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AssociatedItemVisInner {
	TypeAlias(TypeAlias),
	Constant(ConstantItem),
	Function(Function),
}
