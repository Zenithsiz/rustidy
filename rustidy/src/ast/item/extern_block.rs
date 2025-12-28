//! Extern block

// Imports
use {
	super::{Function, MacroInvocationSemi, StaticItem, function::Abi},
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			delimited::Braced,
			token,
			vis::Visibility,
			with_attrs::{WithInnerAttributes, WithOuterAttributes},
		},
	},
};

/// `ExternBlock`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternBlock {
	pub unsafe_: Option<token::Unsafe>,
	pub extern_: token::Extern,
	pub abi:     Option<Abi>,
	pub inner:   Braced<WithInnerAttributes<Vec<ExternalItem>>>,
}

/// `ExternalItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItem(pub WithOuterAttributes<ExternalItemInner>);

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ExternalItemInner {
	Macro(MacroInvocationSemi),
	Static(ExternalItemStatic),
	Function(ExternalItemFunction),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItemStatic {
	pub vis:     Option<Visibility>,
	pub static_: StaticItem,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItemFunction {
	pub vis:      Option<Visibility>,
	pub function: Function,
}
