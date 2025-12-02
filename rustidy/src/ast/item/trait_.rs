//! Trait statement

// Imports
use {
	super::{
		ConstantItem,
		Function,
		MacroInvocationSemi,
		TypeAlias,
		Visibility,
		function::{GenericParams, TypeParamBounds},
	},
	crate::{
		Format,
		ast::{attr::InnerAttrOrDocComment, ident::Ident, token, with_attrs::WithOuterAttributes},
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
	trait_:   token::Trait,
	#[parse(fatal)]
	ident:    Ident,
	generics: Option<GenericParams>,
	bounds:   Option<TraitColonBounds>,
	open:     token::BracesOpen,
	body:     TraitBody,
	close:    token::BracesClose,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitColonBounds {
	colon:  token::Colon,
	bounds: Option<TypeParamBounds>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitBody {
	attrs: Vec<InnerAttrOrDocComment>,
	items: Vec<AssociatedItem>,
}

/// `AssociatedItem`
pub type AssociatedItem = WithOuterAttributes<AssociatedItemInner>;

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
	vis:   Option<Visibility>,
	inner: AssociatedItemVisInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum AssociatedItemVisInner {
	TypeAlias(TypeAlias),
	Constant(ConstantItem),
	Function(Function),
}
