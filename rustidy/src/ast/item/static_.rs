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
	#[format(and_with(expr = Format::prefix_ws_set_single, if = self.safety.is_some()))]
	pub static_: token::Static,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub mut_:    Option<token::Mut>,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ident:   Identifier,
	#[format(and_with = Format::prefix_ws_remove)]
	pub colon:   token::Colon,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub ty:      Type,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub value:   Option<StaticItemValue>,
	#[format(and_with = Format::prefix_ws_remove)]
	pub semi:    token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItemValue {
	pub eq:    token::Eq,
	#[format(and_with = Format::prefix_ws_set_single)]
	pub value: Expression,
}
