//! Static item

// Imports
use {
	super::function::ItemSafety,
	crate::{
		Format,
		Parse,
		Print,
		ast::{expr::Expression, ident::Identifier, token, ty::Type},
	},
};

/// `StaticItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItem {
	pub safety:  Option<ItemSafety>,
	pub static_: token::Static,
	pub mut_:    Option<token::Mut>,
	pub ident:   Identifier,
	pub colon:   token::Colon,
	pub ty:      Type,
	pub value:   Option<StaticItemValue>,
	pub semi:    token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItemValue {
	pub eq:    token::Eq,
	pub value: Expression,
}
