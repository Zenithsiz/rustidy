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
			with_attrs::{WithInnerAttributes, WithOuterAttributes},
		},
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
	// Note: Nightly-only
	pub auto:     Option<token::Auto>,
	pub trait_:   token::Trait,
	#[parse(fatal)]
	pub ident:    Identifier,
	pub generics: Option<GenericParams>,
	pub bounds:   Option<TraitColonBounds>,
	pub where_:   Option<WhereClause>,
	pub body:     TraitBody,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum TraitBody {
	// Note: Nightly-only
	Eq(TraitBodyEq),
	Full(Braced<WithInnerAttributes<TraitBodyFull>>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBodyEq {
	pub eq:     token::Eq,
	pub bounds: Option<TypeParamBounds>,
	pub semi:   token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBodyFull(pub Vec<AssociatedItem>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitColonBounds {
	pub colon:  token::Colon,
	pub bounds: Option<TypeParamBounds>,
}

/// `AssociatedItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct AssociatedItem(pub WithOuterAttributes<AssociatedItemInner>);

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
