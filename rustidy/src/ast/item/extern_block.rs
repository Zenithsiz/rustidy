//! Extern block

// Imports
use {
	super::{Function, MacroInvocationSemi, StaticItem, function::Abi},
	crate::{
		Format,
		Parse,
		Print,
		ast::{
			attr::InnerAttrOrDocComment,
			delimited::Braced,
			token,
			vis::Visibility,
			with_attrs::WithOuterAttributes,
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
	pub inner:   Braced<ExternBlockInner>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternBlockInner {
	attrs: Vec<InnerAttrOrDocComment>,
	items: Vec<ExternalItem>,
}

/// `ExternalItem`
pub type ExternalItem = WithOuterAttributes<ExternalItemInner>;

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
	vis:     Option<Visibility>,
	static_: StaticItem,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ExternalItemFunction {
	vis:      Option<Visibility>,
	function: Function,
}
