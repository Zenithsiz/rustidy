//! Static item

// Imports
use {
	super::function::ItemSafety,
	crate::{
		Format,
		Parse,
		Print,
		ast::{expr::Expression, ident::Ident, token, ty::Type},
	},
};

/// `StaticItem`
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItem {
	safety:  Option<ItemSafety>,
	static_: token::Static,
	mut_:    Option<token::Mut>,
	ident:   Ident,
	colon:   token::Colon,
	ty:      Type,
	value:   Option<StaticItemValue>,
	semi:    token::Semi,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Parse, Format, Print)]
pub struct StaticItemValue {
	eq:    token::Eq,
	value: Expression,
}
