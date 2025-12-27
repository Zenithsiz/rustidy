//! Module item

// Imports
use {
	super::Item,
	crate::{
		Format,
		Parse,
		Print,
		ast::{delimited::Braced, ident::Identifier, token, with_attrs::WithInnerAttributes},
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
	ident:   Identifier,
	inner:   ModuleInner,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub enum ModuleInner {
	None(token::Semi),
	Def(Braced<WithInnerAttributes<Vec<Item>>>),
}
