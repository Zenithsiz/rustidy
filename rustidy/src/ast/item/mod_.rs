//! Module item

// Imports
use {
	super::Item,
	crate::{
		Format,
		Parse,
		Print,
		ast::{attr::InnerAttrOrDocComment, delimited::Braced, ident::Ident, token},
	},
};

/// `Module`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
#[parse(name = "module declaration")]
pub struct Module {
	unsafe_: Option<token::Unsafe>,
	mod_:    token::Mod,
	#[parse(fatal)]
	ident:   Ident,
	inner:   ModuleInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ModuleInner {
	None(token::Semi),
	Def(Braced<ModuleDefInner>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct ModuleDefInner {
	attrs: Vec<InnerAttrOrDocComment>,
	items: Vec<Item>,
}
