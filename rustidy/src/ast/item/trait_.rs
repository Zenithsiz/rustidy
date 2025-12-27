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
	trait_:   token::Trait,
	#[parse(fatal)]
	ident:    Identifier,
	generics: Option<GenericParams>,
	bounds:   Option<TraitColonBounds>,
	body:     Braced<WithInnerAttributes<Vec<AssociatedItem>>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct TraitColonBounds {
	colon:  token::Colon,
	bounds: Option<TypeParamBounds>,
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
